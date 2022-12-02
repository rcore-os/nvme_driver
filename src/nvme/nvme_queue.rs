use alloc::slice;
use core::marker::PhantomData;
use volatile::Volatile;

use super::NvmeCommonCommand;
use super::NvmeCompletion;
use super::NVME_QUEUE_DEPTH;

use crate::dma::DmaAllocator;

const PAGE_SIZE: usize = 4096;

#[derive(Debug)]
pub struct NvmeQueue<D: DmaAllocator> {
    dma_data: PhantomData<D>,

    pub qid: usize,

    pub sq: &'static mut [Volatile<NvmeCommonCommand>],
    pub cq: &'static mut [Volatile<NvmeCompletion>],

    // doorbell register offset of bar address
    pub db_offset: usize,

    pub q_depth: usize,

    pub cq_head: usize,
    pub cq_phase: usize,

    pub sq_tail: usize,
    pub last_sq_tail: usize,

    pub sq_pa: usize,
    pub cq_pa: usize,
    pub data_pa: usize,
}

impl<D: DmaAllocator> NvmeQueue<D> {
    pub fn new(qid: usize, db_offset: usize) -> Self {
        let data_va = D::dma_alloc(PAGE_SIZE * 4);
        let sq_va = D::dma_alloc(NVME_QUEUE_DEPTH*64);
        let cq_va = D::dma_alloc(NVME_QUEUE_DEPTH*16);

        let data_pa = D::virt_to_phys(data_va);
        let sq_pa = D::virt_to_phys(sq_va);
        let cq_pa = D::virt_to_phys(cq_va);

        let submit_queue = unsafe {
            slice::from_raw_parts_mut(sq_va as *mut Volatile<NvmeCommonCommand>, NVME_QUEUE_DEPTH*64)
        };

        let complete_queue = unsafe {
            slice::from_raw_parts_mut(cq_va as *mut Volatile<NvmeCompletion>, NVME_QUEUE_DEPTH*16)
        };


        NvmeQueue {
            dma_data: PhantomData,
            sq: submit_queue,
            cq: complete_queue,
            db_offset,
            q_depth: NVME_QUEUE_DEPTH,
            qid,
            cq_head: 0,
            cq_phase: 1,
            sq_tail: 0,
            last_sq_tail: 0,
            sq_pa,
            cq_pa,
            data_pa,
        }
    }

    pub fn nvme_init_queue(&mut self) {
        self.cq_head = 0;
        self.cq_phase = 1;
        self.sq_tail = 0;
        self.last_sq_tail = 0;
    }
}
