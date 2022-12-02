pub trait IrqController {
    fn disable_irq(irq_num: usize);

    fn enable_irq(irq_num: usize);
}
