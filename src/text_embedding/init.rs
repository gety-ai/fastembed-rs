//! Initialization options for the text embedding models.
//!

use crate::{
    common::{TokenizerFiles, DEFAULT_CACHE_DIR},
    pooling::Pooling,
    EmbeddingModel, QuantizationMode,
};
use ort::{
    execution_providers::ExecutionProviderDispatch,
    session::{builder::GraphOptimizationLevel, Session},
};
use std::{
    num::NonZero,
    path::{Path, PathBuf},
};
use tokenizers::Tokenizer;

use super::{DEFAULT_EMBEDDING_MODEL, DEFAULT_MAX_LENGTH};

/// Options for initializing the TextEmbedding model
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct InitOptions {
    pub model_name: EmbeddingModel,
    pub execution_providers: Vec<ExecutionProviderDispatch>,
    pub max_length: usize,
    pub cache_dir: PathBuf,
    pub show_download_progress: bool,
    /// parallel execution maximum number of threads, only active when parallel_execution is true
    pub node_thread_nums: NonZero<usize>,
    pub graph_thread_nums: NonZero<usize>,
    pub parallel_execution: bool,
    pub optimization_level: Option<GraphOptimizationLevelWrap>,
    pub profiling_output: Option<PathBuf>,
}

impl InitOptions {
    /// Create a new InitOptions with the given model name
    pub fn new(model_name: EmbeddingModel) -> Self {
        Self {
            model_name,
            ..Default::default()
        }
    }

    /// Set the maximum length of the input text
    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = max_length;
        self
    }

    /// Set the cache directory for the model files
    pub fn with_cache_dir(mut self, cache_dir: PathBuf) -> Self {
        self.cache_dir = cache_dir;
        self
    }

    /// Set the execution providers for the model
    pub fn with_execution_providers(
        mut self,
        execution_providers: Vec<ExecutionProviderDispatch>,
    ) -> Self {
        self.execution_providers = execution_providers;
        self
    }

    /// Set whether to show download progress
    pub fn with_show_download_progress(mut self, show_download_progress: bool) -> Self {
        self.show_download_progress = show_download_progress;
        self
    }

    /// Set the node number of threads for parallel execution
    pub fn with_node_thread_nums(mut self, thread_nums: NonZero<usize>) -> Self {
        self.node_thread_nums = thread_nums;
        self
    }

    /// Set the graph number of threads for parallel execution
    pub fn with_graph_thread_nums(mut self, thread_nums: NonZero<usize>) -> Self {
        self.graph_thread_nums = thread_nums;
        self
    }

    /// Set whether to use parallel execution
    pub fn with_parallel_execution(mut self, parallel_execution: bool) -> Self {
        self.parallel_execution = parallel_execution;
        if !parallel_execution {
            self.node_thread_nums = NonZero::new(1).unwrap();
            self.graph_thread_nums = NonZero::new(1).unwrap();
        }
        self
    }

    /// Set the optimization level for the model
    pub fn with_optimization_level(mut self, optimization_level: GraphOptimizationLevel) -> Self {
        self.optimization_level = Some(optimization_level.into());
        self
    }

    /// Set ProfilingOutput
    pub fn with_profiling_output(mut self, profiling_output: &Path) -> Self {
        self.profiling_output = Some(profiling_output.to_path_buf());
        self
    }
}

impl Default for InitOptions {
    fn default() -> Self {
        let thread_nums = std::thread::available_parallelism().unwrap_or(NonZero::new(1).unwrap());
        Self {
            model_name: DEFAULT_EMBEDDING_MODEL,
            execution_providers: Default::default(),
            max_length: DEFAULT_MAX_LENGTH,
            cache_dir: Path::new(DEFAULT_CACHE_DIR).to_path_buf(),
            show_download_progress: true,
            node_thread_nums: thread_nums,
            graph_thread_nums: thread_nums,
            parallel_execution: true,
            optimization_level: None,
            profiling_output: None,
        }
    }
}

#[derive(Debug)]
pub struct GraphOptimizationLevelWrap(GraphOptimizationLevel);
impl Clone for GraphOptimizationLevelWrap {
    fn clone(&self) -> Self {
        Self(match self.0 {
            GraphOptimizationLevel::Level1 => GraphOptimizationLevel::Level1,
            GraphOptimizationLevel::Level2 => GraphOptimizationLevel::Level2,
            GraphOptimizationLevel::Level3 => GraphOptimizationLevel::Level3,
            GraphOptimizationLevel::Disable => GraphOptimizationLevel::Disable,
        })
    }
}

impl From<GraphOptimizationLevel> for GraphOptimizationLevelWrap {
    fn from(level: GraphOptimizationLevel) -> Self {
        Self(level)
    }
}

impl From<GraphOptimizationLevelWrap> for GraphOptimizationLevel {
    fn from(level: GraphOptimizationLevelWrap) -> Self {
        level.0
    }
}

/// Options for initializing UserDefinedEmbeddingModel
///
/// Model files are held by the UserDefinedEmbeddingModel struct
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct InitOptionsUserDefined {
    pub execution_providers: Vec<ExecutionProviderDispatch>,
    pub max_length: usize,
    /// parallel execution maximum number of threads, only active when parallel_execution is true
    pub node_thread_nums: NonZero<usize>,
    pub graph_thread_nums: NonZero<usize>,
    pub parallel_execution: bool,
    pub optimization_level: Option<GraphOptimizationLevelWrap>,
    pub profiling_output: Option<PathBuf>,
}

impl InitOptionsUserDefined {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_execution_providers(
        mut self,
        execution_providers: Vec<ExecutionProviderDispatch>,
    ) -> Self {
        self.execution_providers = execution_providers;
        self
    }

    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = max_length;
        self
    }

    pub fn with_node_thread_nums(mut self, thread_nums: NonZero<usize>) -> Self {
        self.node_thread_nums = thread_nums;
        self
    }

    pub fn with_graph_thread_nums(mut self, thread_nums: NonZero<usize>) -> Self {
        self.graph_thread_nums = thread_nums;
        self
    }

    pub fn with_parallel_execution(mut self, parallel_execution: bool) -> Self {
        self.parallel_execution = parallel_execution;
        if !parallel_execution {
            self.node_thread_nums = NonZero::new(1).unwrap();
            self.graph_thread_nums = NonZero::new(1).unwrap();
        }

        self
    }

    pub fn with_optimization_level(mut self, optimization_level: GraphOptimizationLevel) -> Self {
        self.optimization_level = Some(optimization_level.into());
        self
    }

    pub fn with_profiling_output(mut self, profiling_output: &Path) -> Self {
        self.profiling_output = Some(profiling_output.to_path_buf());
        self
    }
}

impl Default for InitOptionsUserDefined {
    fn default() -> Self {
        let thread_nums = std::thread::available_parallelism().unwrap_or(NonZero::new(1).unwrap());
        Self {
            execution_providers: Default::default(),
            max_length: DEFAULT_MAX_LENGTH,
            node_thread_nums: thread_nums,
            graph_thread_nums: thread_nums,
            parallel_execution: true,
            optimization_level: None,
            profiling_output: None,
        }
    }
}

/// Convert InitOptions to InitOptionsUserDefined
///
/// This is useful for when the user wants to use the same options for both the default and user-defined models
impl From<InitOptions> for InitOptionsUserDefined {
    fn from(options: InitOptions) -> Self {
        InitOptionsUserDefined {
            execution_providers: options.execution_providers,
            max_length: options.max_length,
            node_thread_nums: options.node_thread_nums,
            graph_thread_nums: options.graph_thread_nums,
            parallel_execution: options.parallel_execution,
            optimization_level: options.optimization_level,
            profiling_output: options.profiling_output,
        }
    }
}

/// Struct for "bring your own" embedding models
///
/// The onnx_file and tokenizer_files are expecting the files' bytes
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserDefinedEmbeddingModel {
    pub onnx_file: Vec<u8>,
    pub tokenizer_files: TokenizerFiles,
    pub pooling: Option<Pooling>,
    pub quantization: QuantizationMode,
}

impl UserDefinedEmbeddingModel {
    pub fn new(onnx_file: Vec<u8>, tokenizer_files: TokenizerFiles) -> Self {
        Self {
            onnx_file,
            tokenizer_files,
            quantization: QuantizationMode::None,
            pooling: None,
        }
    }

    pub fn with_quantization(mut self, quantization: QuantizationMode) -> Self {
        self.quantization = quantization;
        self
    }

    pub fn with_pooling(mut self, pooling: Pooling) -> Self {
        self.pooling = Some(pooling);
        self
    }
}

/// Rust representation of the TextEmbedding model
pub struct TextEmbedding {
    pub tokenizer: Tokenizer,
    pub(crate) pooling: Option<Pooling>,
    pub(crate) session: Session,
    pub(crate) enable_profiling: bool,
    pub(crate) need_token_type_ids: bool,
    pub(crate) quantization: QuantizationMode,
}
