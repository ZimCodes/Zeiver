pub mod olaindex;
#[derive(PartialEq,Debug)]
pub enum ODMethod {
    OLAINDEX,
    Generic
}
/*Determine the od type from URL*/
pub fn od_type_from_url(url:&str)-> ODMethod {
    if olaindex::OLAINDEX::hash_query(url){
        ODMethod::OLAINDEX
    }else{
        ODMethod::Generic
    }
}
/*Determine od type from HTML Document */
pub fn od_type_from_document(res:&str) -> ODMethod {
    if olaindex::OLAINDEX::has_data_route(res){
        ODMethod::OLAINDEX
    }else{
        ODMethod::Generic
    }
}
