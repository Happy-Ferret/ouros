use x86_64::structures::idt::Idt;
use x86_64::structures::idt::ExceptionStackFrame; 
use memory::MemoryController ; 
use x86_64::structures::tss::TaskStateSegment ; 
use x86_64::VirtualAddress;

use spin::Once ; 

static TSS: Once<TaskStateSegment> = Once::new() ; 
static GDT: Once<gdt::Gdt> = Once::new() ; 

mod gdt ; 


lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        
        unsafe{
            idt.double_fault.set_handler_fn(double_fault_handler)
                                .set_stack_index(0 as u16) ;     
        }
        
        idt
    };
}

pub fn init(memory_controller: &mut MemoryController) {

    let double_fault_stack = memory_controller.alloc_stack(1)
        .expect("could not allocate double fault stack"); 


    let tss = TSS.call_once(|| {
        let mut tss = TaskStateSegment::new() ; 
        tss.interrupt_stack_table[0] = VirtualAddress(double_fault_stack.top()) ; 
        tss 
    }); 
    
    use x86_64::structures::gdt::SegmentSelector;
    use x86_64::instructions::segmentation::set_cs;
    use x86_64::instructions::tables::load_tss;

    let mut code_selector = SegmentSelector(0) ; 
    let mut tss_selector  = SegmentSelector(0) ; 
    let gdt = GDT.call_once(|| {
        let mut gdt = gdt::Gdt::new() ;
        code_selector = gdt.add_entry(gdt::Descriptor::kernel_code_segment()) ; 
        tss_selector  = gdt.add_entry(gdt::Descriptor::tss_segment(&tss)) ; 
        gdt 
    }); 
    
    gdt.load() ; 

    unsafe {
        set_cs(code_selector) ; 
        load_tss(tss_selector); 
    }


    IDT.load();
}


extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame, _error_code: u64){
	println!("EXCEPTION: DOUBLE FAULT \n{:?}\n{:?}", _error_code, stack_frame) ; 
	loop{}
}