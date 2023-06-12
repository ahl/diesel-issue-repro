use std::marker::PhantomData;

use diesel::{backend::Backend, query_builder::bind_collector::RawBytesBindCollector};

pub struct Test<DB>(PhantomData<DB>)
where
    DB: Backend<BindCollector = RawBytesBindCollector<DB>>;
