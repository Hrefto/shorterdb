pub fn bytes_to_string(key: &[u8])-> String{
    let mut out = "".to_string();
    for i in key{
        out=out+i.to_string().as_str()+",";
    }
    out.to_string()
}