use criterion::Throughput;
use core::arch::x86_64::_rdtsc;

pub struct CpuCycles;

#[derive(Clone, Copy, Debug)]
pub struct Cycles(pub u64);

impl criterion::measurement::Measurement for CpuCycles {
    type Intermediate = u64;
    type Value = Cycles;

    fn start(&self) -> Self::Intermediate {
        unsafe { _rdtsc() }
    }

    fn end(&self, start: Self::Intermediate) -> Self::Value {
        Cycles(unsafe { _rdtsc() - start })
    }

    fn add(&self, a: &Cycles, b: &Cycles) -> Cycles {
        Cycles(a.0 + b.0)
    }

    fn zero(&self) -> Cycles {
        Cycles(0)
    }

    fn to_f64(&self, value: &Cycles) -> f64 {
        value.0 as f64
    }

    fn formatter(&self) -> &dyn criterion::measurement::ValueFormatter {
        &CyclesFormatter
    }
}

struct CyclesFormatter;

impl criterion::measurement::ValueFormatter for CyclesFormatter {
    fn format_value(&self, value: f64) -> String {
        format!("{:.0} cycles", value)
    }

    fn scale_values(&self, _typical: f64, values: &mut [f64]) -> &'static str {
        for v in values.iter_mut() {
            *v *= 1.0;
        }
        "cycles"
    }

    fn scale_throughputs(&self, _typical: f64, _throughput: &Throughput, values: &mut [f64]) -> &'static str {
        for v in values.iter_mut() {
            *v *= 1.0;
        }
        "cycles"
    }

    fn scale_for_machines(&self, values: &mut [f64]) -> &'static str {
        for v in values.iter_mut() {
            *v *= 1.0;
        }
        "cycles"
    }
}