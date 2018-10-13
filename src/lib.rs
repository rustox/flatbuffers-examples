extern crate mio;
extern crate procfs;
extern crate flatbuffers;
extern crate bytes;

use procfs::LoadAverage as PLA;
use flatbuffers::FlatBufferBuilder;

#[path="procinfo_generated.rs"] pub mod procinfo_generated;

pub use procinfo_generated::rustox::procinfo::{
  LoadAverage, Proc, ProcArgs, ProcBuilder,
  get_root_as_proc
};

pub fn test_flat_buffer() {
  let la = PLA::new().unwrap();
  println!("{:?}", la);

  let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);

  let fbla = LoadAverage::new(la.one, la.five, la.fifteen,
                              la.cur, la.max, la.latest_pid);

  let procinfo = Proc::create(&mut builder, &ProcArgs{
    load : Some(&fbla)
  });

  builder.finish(procinfo, None);

  let buf = builder.finished_data();

  let proc = get_root_as_proc(buf);

  println!("{:?}", proc.load().unwrap());
}

