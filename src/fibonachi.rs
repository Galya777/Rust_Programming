pub fn fib(n: u32) -> u32 {
if n==0 {
return 0;
    
}else if n==1 { 
return 1;
    
}else {
return fib(n-2) +fib(n-1);
}
}
