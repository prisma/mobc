use metrics::{describe_counter, describe_gauge, describe_histogram};

pub const OPENED_TOTAL: &str = "prisma_pool_connections_opened_total";
pub const CLOSED_TOTAL: &str = "prisma_pool_connections_closed_total";
pub const OPEN_CONNECTIONS: &str = "prisma_pool_connections_open";

pub const ACTIVE_CONNECTIONS: &str = "prisma_pool_connections_busy";
pub const IDLE_CONNECTIONS: &str = "prisma_pool_connections_idle";
pub const WAIT_COUNT: &str = "prisma_client_queries_wait";
pub const WAIT_DURATION: &str = "prisma_client_queries_wait_histogram_ms";

pub fn describe_metrics() {
    describe_counter!(OPENED_TOTAL, "Total number of Pool Connections opened");
    describe_counter!(CLOSED_TOTAL, "Total number of Pool Connections closed");

    describe_gauge!(
        OPEN_CONNECTIONS,
        "Number of currently open Pool Connections"
    );

    describe_gauge!(
        ACTIVE_CONNECTIONS,
        "Number of currently busy Pool Connections (executing a datasource query)"
    );

    describe_gauge!(
        IDLE_CONNECTIONS,
        "Number of currently unused Pool Connections (waiting for the next datasource query to run)"
    );

    describe_gauge!(
        WAIT_COUNT,
        "Number of Prisma Client queries currently waiting for a connection"
    );

    describe_histogram!(
        WAIT_DURATION,
        "Histogram of the wait time of all Prisma Client Queries in ms"
    );
}
