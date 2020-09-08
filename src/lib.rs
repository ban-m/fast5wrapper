#![crate_name = "fast5wrapper"]
#![crate_type = "lib"]
#![warn(missing_docs)]

//!Wrapper function to extract the signal data from specified fast5 file.
//!
//!In the benckmarking, it is of course inevitable 
//!to open fast5 file and get the signal data from it.
//!To get it done, one should use the HDF-API to access the 
//!group/attribute/data in the fast5 file.
//!However, that is a huge library and I decided to 
//!use more dirty tech: to call python from Rust.
//!The elapsed time is 13,519,219 ns/file (+/- 4,284,512) currently.
extern crate cpython;
pub mod result;
use std::str;
use std::vec::Vec;
use std::f32;
use cpython::{Python,PyErr, ObjectProtocol,PyTuple,PyString,PythonObject,PyList};

///#  Example
///
///    let path = "/path/to/fast5/file.fast5";
///    let result = get_event(path).unwrap();
///    for row in result {
///        for column in row {
///            print!("{},",column)
///        }
///        println!("");
///    }
///
///Open the specified fast5 file and 
///extract the event data.
///
///It is important to note that this function 
///invoke python interpreter inside the procedure,
///and some packages are required to execute 
///properly.
///
///+ ont_fast5_api
///+ h5py
///Input fast5 file probably can be R7/R9 fast5 file.
///
/// If you want to specify the skip and the take length,
/// use get_event_for() instead.
pub fn get_event(path:&str)-> result::Result<Vec<[f32;4]>>{
    let bignum = 1_000_000_000;//1gb event.
    get_event_for(path,0,bignum)
}

/// Open the specified fast5 file and extract data for data[skip..skip+take].
/// Note that this method is *faster* than just use get_event and
/// trimming the result.
pub fn get_event_for(path:&str,skip:usize,take:usize)-> result::Result<Vec<[f32;4]>>{
    get_event_for_oldmodel(path,skip,take)
        .or(get_event_for_newmodel(path,skip,take))
}

fn get_event_for_oldmodel(path:&str,skip:usize,take:usize)-> result::Result<Vec<[f32;4]>>{
    let gil = Python::acquire_gil();
    let python = gil.python();
    let event_detection = python.import("ont_fast5_api.analysis_tools.event_detection")?;
    let path = PyString::new(python,path).into_object();
    let arg = PyTuple::new(python,&[path]);
    
    let list :PyList = event_detection
        .get(python,"EventDetectionTools")?
    .call(python,arg,None)?
    .call_method(python,
                 "get_event_data",
                 PyTuple::new(python,&[]),
                 None)?
    .get_item(python,0)?
    .call_method(python,
                 "tolist",
                 PyTuple::new(python,&[]),
                 None)?
    .cast_into(python)
        .map_err(|_|PyErr::fetch(python))?;
    let mut res = vec![];
    for pytuple in list.iter(python).skip(skip).take(take){
        let start:f32 = pytuple.get_item(python,0)?.extract(python)?;
        let length:f32 = pytuple.get_item(python,1)?.extract(python)?;
        let mean:f32 = pytuple.get_item(python,2)?.extract(python)?;
        let stdv:f32 = pytuple.get_item(python,3)?.extract(python)?;
        res.push([start,length,mean,stdv]);
    }
    Ok(res)
}

fn get_event_for_newmodel(path:&str,skip:usize,take:usize)->result::Result<Vec<[f32;4]>>{
    let gil =  Python::acquire_gil();
    let python = gil.python();
    let h5py = python.import("h5py")?;
    let path = PyString::new(python,path).into_object();
    let read_only = PyString::new(python,"r").into_object();
    let arg = PyTuple::new(python,&[path,read_only]);
    let key = PyString::new(python,"/Analyses/Basecall_1D_000/BaseCalled_template/Events")
        .into_object();
    let key = PyTuple::new(python,&[key]);
    let list:PyList = h5py.get(python,"File")?
    .call(python,arg,None)?
    .call_method(python,"get",key,None)?
    .getattr(python,"value")?
    .call_method(python,
                 "tolist",
                 PyTuple::new(python,&[]),
                 None)?
    .cast_into(python)
        .map_err(|_|PyErr::fetch(python))?;
    let mut res = vec![];
    for pytuple in list.iter(python).skip(skip).take(take){
        let mean:f32 = pytuple.get_item(python,0)?.extract(python)?;
        let start:f32 = pytuple.get_item(python,1)?.extract(python)?;
        let stdv:f32 = pytuple.get_item(python,2)?.extract(python)?;
        let length:f32 = pytuple.get_item(python,3)?.extract(python)?;
        res.push([start,length,mean,stdv]);
    }
    Ok(res)
}

/// Open fast5 file and extract its read id
pub fn get_read_id(path:&str)->result::Result<String>{
    let gil = Python::acquire_gil();
    let python = gil.python();
    let path = PyString::new(python,path).into_object();
    let arg = PyTuple::new(python,&[path]);
    let fast5info = match python.import("ont_fast5_api.fast5_info"){
        Ok(res) => res,
        Err(why) => return Err(result::Error::from(why)),
    };
    let fast5info = fast5info.get(python,"Fast5Info")?;
    let data = fast5info.call(python,arg,None)?;
    let data = data.getattr(python,"read_info")?;
    let data =  data.get_item(python,0)?;
    let res = data.getattr(python,"read_id")?.extract(python)?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
    }
    #[test]
    fn test(){
        let res = get_read_id("/home/ban-m/work/minidata/read703.fast5").unwrap();
        assert_eq!(res,"e78a1934-a540-42e7-bf36-bd321c86b24c".to_string());
    }
}



// /// This method is to extract raw signal from fast5 file.
// pub fn get_signal_for(path:&str,skip:usize,take:usize)->result::Result<Vec<u16>>{
//     let gil = Python::acquire_gil();
//     let python = gil.python();
//     let h5py = python.import("h5py")?;
//     let path = PyString::new(python,path).into_object();
//     let read_only = PyString::new(python,"r").into_object();
//     let arg = PyTuple::new(python,&[path,read_only]);
//     let read_num = 1;
//     let key = PyString::new(python,format!("/Raw/Reads/{}/Signal",read_num))
//         .into_object();
//     let key = PyTuple::new(python,&[key]);
//     let list:PyList = h5py.get(python,"File")?
//     .call(python,arg,None)?
//     .call_method(python,"get",key,None)?
//     .getattr(python,"value")?
//     .call_method(python,
//                  "tolist",
//                  PyTuple::new(python,&[]),
//                  None)?
//     .cast_into(python)
//         .map_err(|_| PyErr::fetch(python))?;
//     let mut res = vec![];
//     for x in list.iter(python).skip(skip).take(take){
//         let x:u16 = x.extract(python)?;
//         res.push(x);
//     }
//     Ok(res)
// }

