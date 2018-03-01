use x86_64::structures::idt::Idt;
use x86_64::structures::idt::ExceptionStackFrame; 
use memory::MemoryController ; 
use x86_64::structures::tss::TaskStateSegment ; 
use x86_64::VirtualAddress;

mod gdt ; 


lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler) ; 
        idt
    };
}

pub fn init(memory_controller: &mut MemoryController) {

    let double_fault_stack = memory_controller.alloc_stack(1)
        .expect("could not allocate double fault stack"); 


    let mut tss = TaskStateSegment::new() ;  
    tss.interrupt_stack_table[0] = VirtualAddress(double_fault_stack.top()) ; 
    
    let gdt = gdt::Gdt::new() ;
    let code_selector = gdt.add_entry(gdt::Descriptor::kernel_code_segment()) ; 
    let tss_selector  = gdt.add_entry(gdt::Descriptor::tss_segment(&tss)) ; 
    gdt.load() ; 

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