mod protos {
    pub mod database {
        tonic::include_proto!("proto.database");
    }

    pub mod recommender {
        tonic::include_proto!("proto.recommender");
    }

    pub mod types {
        tonic::include_proto!("proto.types");
    }
}

pub mod database;
pub mod recommender;
pub mod types;
