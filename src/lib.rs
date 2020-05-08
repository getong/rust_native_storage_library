use libc::{size_t};
use rocksdb::DB;
use std::any::type_name;
use std::convert::TryInto;
use std::ffi::CString;
use std::os::raw::c_char;
use std::slice;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[no_mangle]
pub extern "C" fn store_byte_array(
    key_array_pointer: *const c_char,
    key_size: size_t,
    value_array_pointer: *const c_char,
    value_size: size_t,
) {
    let key = unsafe {
        assert!(!key_array_pointer.is_null());

        slice::from_raw_parts(key_array_pointer as *const _, key_size as usize)
    };
    let value = unsafe {
        assert!(!value_array_pointer.is_null());

        slice::from_raw_parts(value_array_pointer as *const _, value_size as usize)
    };
    //println!("Key from raw parts has a type of: {:?}", type_of(key));
    println!("Storing data, please wait ...");
    let path = "/media/nvme/ssvm_database";
    println!("Database path: {:?}", path);
    let db = DB::open_default(path).unwrap();
    println!("Database instance: {:?}", db);
    //let u8slice_k = unsafe { &*(&key as *const _ as *const [u8]) };
    //let u8slice_v = unsafe { &*(&value as *const _ as *const [u8]) };
    //db.put(u8slice_k, u8slice_v).unwrap();
    db.put(key, value).unwrap();
    println!("Item added to database");
}

#[no_mangle]
pub extern "C" fn get_byte_array_pointer(
    _key_array_pointer: *const c_char,
    _key_size: size_t,
) -> *mut c_char {
    let _key = unsafe {
        assert!(!_key_array_pointer.is_null());

        slice::from_raw_parts(_key_array_pointer as *const _, _key_size as usize)
    };
    println!("Loading data, please wait ...");
    let path = "/media/nvme/ssvm_database";
    println!("Database path: {:?}", path);
    let db = DB::open_default(path).unwrap();
    println!("Database instance: {:?}", db);
    //let u8slice = unsafe { &*(&_key as *const _ as *const [u8]) };
    //let loaded_data = db.get(&u8slice).unwrap();
    let loaded_data = db.get(&_key).unwrap();
    println!("Loaded data: {:?}", loaded_data);
    let ptr: *mut c_char = loaded_data.unwrap().as_ptr() as *mut i8;
    println!("Pointer: {:?}", ptr);
    ptr
}

#[no_mangle]
pub extern "C" fn get_byte_array_length(
    _key_array_pointer: *const c_char,
    _key_size: size_t,
) -> size_t {
    let _key = unsafe {
        assert!(!_key_array_pointer.is_null());

        slice::from_raw_parts(_key_array_pointer as *const _, _key_size as usize)
    };
    println!("Loading data, please wait ...");
    let path = "/media/nvme/ssvm_database";
    println!("Database path: {:?}", path);
    let db = DB::open_default(path).unwrap();
    println!("Database instance: {:?}", db);
    //let u8slice = unsafe { &*(&_key as *const _ as *const [u8]) };
    //let loaded_data = db.get(&u8slice).unwrap();
    let loaded_data = db.get(&_key).unwrap();
    println!("Loaded data: {:?}", loaded_data);
    let size: size_t = loaded_data.unwrap().len().try_into().unwrap();
    println!("Size: {:?}", size);
    size
}

#[no_mangle]
pub extern "C" fn free_byte_array_pointer(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        // TODO find best way to deallocate pointer
        CString::from_raw(s)
    };
}

// The code below worked really well
// i.e. store/load (using CStr as string) and store/load (using CStr as bytes)
// However these original functions will be excluded from the API
// Reason being any use of CStr means that the data is not allowed to have \0 \0x00 nul
// We need to accomodate these values as valid input. The DB will put and get arbitrary byte arrays so we need to ensure that this API does not offer less than that
/*
#[no_mangle]
pub extern "C" fn store_bytes(_key: *const c_char, _value: *const c_char) {
    let c_str_key = unsafe {
        assert!(!_key.is_null());

        CStr::from_ptr(_key)
    };
    let c_str_value = unsafe {
        assert!(!_value.is_null());

        CStr::from_ptr(_value)
    };
    println!("Storing data, please wait ...");
    let path = "/media/nvme/ssvm_database";
    println!("Database path: {:?}", path);
    let db = DB::open_default(path).unwrap();
    println!("Database instance: {:?}", db);
    db.put(c_str_key.to_bytes(), c_str_value.to_bytes())
        .unwrap();
    println!("Item added to database");
}

#[no_mangle]
pub extern "C" fn load_bytes(_key: *const c_char) -> *mut c_char {
    let c_str_key = unsafe {
        assert!(!_key.is_null());

        CStr::from_ptr(_key)
    };
    println!("Loading data, please wait ...");
    let path = "/media/nvme/ssvm_database";
    println!("Database path: {:?}", path);
    let db = DB::open_default(path).unwrap();
    println!("Database instance: {:?}", db);
    CString::new(db.get(c_str_key.to_bytes()).unwrap().unwrap())
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn store_data(_key: *const c_char, _value: *const c_char) {
    println!("Storing data, please wait ...");
    let c_str = unsafe {
        assert!(!_value.is_null());

        CStr::from_ptr(_value)
    };
    let _value_as_string = c_str.to_str().unwrap();

    let c_str_key = unsafe {
        assert!(!_key.is_null());

        CStr::from_ptr(_key)
    };
    let _key_as_string = c_str_key.to_str().unwrap();
    let path = "/media/nvme/ssvm_database";
    println!("Database path: {:?}", path);
    let db = DB::open_default(path).unwrap();
    println!("Database instance: {:?}", db);
    let mut opts = Options::default();
    opts.increase_parallelism(3);
    opts.create_if_missing(true);
    //println!("Database options are set");
    db.put(_key_as_string, _value_as_string).unwrap();
    println!("Item added to database");
}

#[no_mangle]
pub extern "C" fn load_data(_key: *const c_char) -> *mut c_char {
    println!("Loading data, please wait ...");
    let c_str_key = unsafe {
        assert!(!_key.is_null());

        CStr::from_ptr(_key)
    };
    let _key_as_string = c_str_key.to_str().unwrap();
    let path = "/media/nvme/ssvm_database";
    //println!("Database path: {:?}", path);*
    let db = DB::open_default(path).unwrap();
    //println!("Database instance: {:?}", db);
    let mut opts = Options::default();
    opts.increase_parallelism(3);
    //println!("Database options are set");
    let db_value_as_vec = db.get(_key_as_string).unwrap().unwrap();
    let db_value_as_cstring = CString::new(db_value_as_vec).unwrap();
    //println!("Value as CString: {:?}", db_value_as_cstring);
    db_value_as_cstring.into_raw()
}

#[no_mangle]
pub extern "C" fn free_pointer(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}
*/
