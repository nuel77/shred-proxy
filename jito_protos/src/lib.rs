pub mod shared {
    tonic::include_proto!("shared");
}

pub mod auth {
    tonic::include_proto!("auth");
}

pub mod shredstream {
    tonic::include_proto!("shredstream");
}

pub mod geyser {
    tonic::include_proto!("geyser");
}

pub mod solana {
    pub mod storage {
        pub mod confirmed_block {
            tonic::include_proto!("solana.storage.confirmed_block");
        }
    }
}
