-- Add migration script here
create table allocator_distribution (
    allocator text not null,
    client text not null,
    num_of_allocations bigint not null,
    sum_of_allocations bigint not null
)