pub trait DmaAllocator {
    fn dma_alloc(size: usize) -> usize;

    fn dma_dealloc(addr: usize, size: usize) -> usize;

    fn phys_to_virt(phys: usize) -> usize;

    fn virt_to_phys(virt: usize) -> usize;
}
