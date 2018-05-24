#[macro_use] extern crate rustler;
#[macro_use] extern crate rustler_codegen;
#[macro_use] extern crate lazy_static;

use rustler::{NifEnv, NifTerm, NifResult, NifError, NifEncoder};
use rustler::resource::ResourceArc;
use std::sync::RwLock;

mod atoms {
    rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

struct Buffer {
    data: RwLock<Vec<u8>>
}

rustler_export_nifs! {
    "Elixir.RustlerTest.Native",
    [
		("add", 2, add),
	 	("create", 1, create_buffer),
	 	("get", 2, get_byte),
	 	("set", 3, set_byte),
	 	("print", 1, print_buffer),
	],
    Some(on_init)
}

fn add<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let num1: i64 = try!(args[0].decode());
    let num2: i64 = try!(args[1].decode());

    Ok((atoms::ok(), num1 + num2).encode(env))
}

fn on_init<'a>(env: NifEnv<'a>, _load_info: NifTerm<'a>) -> bool {
    resource_struct_init!(Buffer, env);
    true
}

fn create_buffer<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let buffer_size: usize = args[0].decode()?;

    let buffer = vec![0; buffer_size];

    let buffer_struct = Buffer {
        data: RwLock::new(buffer)
    };

    Ok((atoms::ok(), ResourceArc::new(buffer_struct)).encode(env))
}

fn get_byte<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
	let buffer: ResourceArc<Buffer> = args[0].decode()?;
	let offset: usize = args[1].decode()?;

    let data = buffer.data.read()
        .map_err(|_| NifError::Atom("Bad Lock"))?;

    Ok((atoms::ok(), data[offset]).encode(env))
}

fn set_byte<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
	let buffer: ResourceArc<Buffer> = args[0].decode()?;
	let offset: usize = args[1].decode()?;
	let byte: u8 = args[2].decode()?;

    let mut data = buffer.data.write()
        .map_err(|_| NifError::Atom("Bad Lock"))?;

    data[offset] = byte;
    Ok((atoms::ok(), offset).encode(env))
}

fn print_buffer<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let buffer: ResourceArc<Buffer> = args[0].decode()?;


    let data = buffer.data.read()
        .map_err(|_| NifError::Atom("Bad Lock"))?;

    println!("{:?}", *data);

    Ok(atoms::ok().encode(env))
}
