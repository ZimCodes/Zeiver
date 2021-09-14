pub mod olaindex;
#[derive(PartialEq,Debug)]
pub enum ODType{
    OLAINDEX,
    General
}
/*Determine the od type from URL*/
pub fn od_type_from_url(url:&str)-> ODType{
    if olaindex::OLAINDEX::hash_query(url){
        ODType::OLAINDEX
    }else{
        ODType::General
    }
}
/*Determine od type from HTML Document */
pub fn od_type_from_document(res:&str) -> ODType{
    if olaindex::OLAINDEX::has_data_route(res){
        ODType::OLAINDEX
    }else{
        ODType::General
    }
}
