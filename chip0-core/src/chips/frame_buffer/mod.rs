pub mod air;
pub mod columns;
pub mod interaction;

#[cfg(feature = "trace-writer")]
use p3_air_util::TraceWriter;
#[cfg(feature = "trace-writer")]
use p3_field::{ExtensionField, Field};

#[cfg(feature = "trace-writer")]
use self::columns::FrameBufferCols;

#[derive(Clone, Debug)]
pub struct FrameBufferChip {
    pub bus_frame_buffer: usize,
    pub bus_range: usize,
}

#[cfg(feature = "trace-writer")]
impl<F: Field, EF: ExtensionField<F>> TraceWriter<F, EF> for FrameBufferChip {
    fn headers(&self) -> Vec<String> {
        FrameBufferCols::<F>::headers()
    }
}
