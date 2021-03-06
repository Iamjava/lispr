use std::ops::{Add,Sub,Div,Mul, Index};

macro_rules! lsp {
    ((+ $a:tt $b:tt)) => {{lsp![$a]+lsp![$b] }};
    ((- $a:tt $b:tt)) => { lsp![$a]-lsp![$b] };
    ((div $a:tt $b:tt)) => { lsp![$a] /lsp![$b] };
    ((eq $a:tt $b:tt)) => { lsp![$a].internal()==lsp![$b].internal() };
    ((mul $a:tt $b:tt)) => { lsp![$a] *lsp![$b] };
    ((print $($a:tt)+ )) =>{println!("{:?}", lsp![$($a)+])};
    ((first $($a:tt)+))=>{{
        let v=lsp![$($a)+];
        v[0]
        }};

    ((rest $($a:tt)+))=>{{
        let mut v=lsp![$($a)+];
        v.remove(0);
        v
        }};

    (#($($a:tt)+))=>{{
        lsp![(#$($a)+)]   
        }};

    ((# $($a:tt)+))=>{{
        let mut v=Vec::new();
        $(v.push(lsp![$a]);)+
        v
        }};

    ((nth $n:tt $($a:tt)+))=>{{
        let v = lsp![$($a)+];
        v[$n]
        }};
    ((if $cond:tt ($($then:tt)+) ($($else:tt)*) ))=>{
        if lsp![$cond] {
            lsp![($($then)+)]
        }else{
             lsp![($($else)*)]
        }
    };
    ((setq $varname:ident $($more:tt)+))=>{
          let $varname = lsp![$($more)+];  
        };

    ((defun $fname:ident ($($paramname:ident)*)  $($body:tt)+))=>{
         //let $fname = |&$($paramname,)*|{lsp![$($body)+]};
         fn $fname($($paramname:&Lispex,)*)->Lispex{
            lsp![$($body)+]
         }
      };

    (($fname:ident $($param:tt)*))=>{
        $fname($(&lsp![$param],)*)
            
        };

    ($varname: ident) => {
       $varname
        };

    ($a:expr)=>{Lispex::NUMBER($a)};
    
}

macro_rules! lsp_program {
    ($($a:tt)+)=>{
        $(lsp![$a];)+
    };
}

macro_rules! impl_ops {
    ($op:tt,$fnname:ident,$implType:ty,$forType:ty,$patern:pat,$out:expr)=>{

impl $op<$implType> for $forType{
    type Output = Lispex;
    fn $fnname(self,o:$implType)->Lispex{
        if let $patern = (self,o){
            Lispex::NUMBER($out)
        }else{
            panic!(stringify!($implType encountered an error with $op))
        }
    }
}

    };
}


fn main() {
    lsp_program![
        (defun square (x) (mul x x))
        (print (square 3))
        (defun fac (x) (if (eq x 1) (+ 1 10) (mul x (fac (- x 1))) ))
        (print (fac 11))
        (print (if (eq 1 0) (1) (+ 1 1111)))
        (setq a #(1 2 3))
        (print a)
        (defun mn (x) (- 100 x))
        (print (mn 3))
    ];
}



#[derive(Debug,Clone,PartialEq)]
enum Lispex{
   ATOM(String),
   LIST(Vec<Lispex>),
   NUMBER(i32)
}

impl Lispex {
    fn internal(&self) -> i32 {
        if let Lispex::NUMBER(a)= self{ 
            *a
        }else{
            0
        }
    }
}




impl Index<usize> for Lispex {
    type Output = Self;

    fn index(&self, n:usize ) -> &Self {
        if let Lispex::LIST(vec)=&self{
            &vec[n]
        }else{
            panic!("Cant index a non indexable thing")
        }
    }
}



impl_ops!(Add,add,Lispex,Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a+b);
impl_ops!(Add,add,Lispex,&Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a+b);
impl_ops!(Add,add,&Lispex,Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a+b);
impl_ops!(Add,add,&Lispex,&Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a+b);


impl_ops!(Sub,sub,Lispex,Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a-b);
impl_ops!(Sub,sub,&Lispex,Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a-b);
impl_ops!(Sub,sub,Lispex,&Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a-b);
impl_ops!(Sub,sub,&Lispex,&Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a-b);


impl_ops!(Mul,mul,Lispex,Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a*b);
impl_ops!(Mul,mul,&Lispex,Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a*b);
impl_ops!(Mul,mul,Lispex,&Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a*b);
impl_ops!(Mul,mul,&Lispex,&Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a*b);


impl_ops!(Div,div,Lispex,Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a/b);
impl_ops!(Div,div,&Lispex,Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a/b);
impl_ops!(Div,div,Lispex,&Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a/b);
impl_ops!(Div,div,&Lispex,&Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a/b);




/*

impl Sub<Lispex> for i32{
    type Output = Lispex;
    fn sub(self,o:Lispex)->Lispex{
        if let (a,Lispex::NUMBER(b))= (self,o){
            Lispex::NUMBER(a-b)
        }else{
            panic!("cant subtract two not addable types")
        }
    }
}

impl Sub<&Lispex> for i32{
    type Output = Lispex;
    fn sub(self,o:&Lispex)->Lispex{
        if let (a,&Lispex::NUMBER(b))= (self,o){
            Lispex::NUMBER(a-b)
        }else{
            panic!("cant subtract two not addable types")
        }
    }
}


impl Mul<Lispex> for i32{
    type Output = Lispex;
    fn mul(self,o:Lispex)->Lispex{
        if let (a,Lispex::NUMBER(b))= (self,o){
            Lispex::NUMBER(a*b)
        }else{
            panic!("cant subtract two not addable types")
        }
    }
}

impl Add<Lispex> for i32{
    type Output = Lispex;
    fn add(self,o:Lispex)->Lispex{
        if let (a,Lispex::NUMBER(b))= (self,o){
            Lispex::NUMBER(a+b)
        }else{
            panic!("cant subtract two not addable types")
        }
    }
}





impl From<i32> for Lispex {
    fn from(s:i32)->Lispex{
        return Lispex::NUMBER(s);
    }
}

impl From<String> for Lispex {
    fn from(s:String)->Lispex{
        return Lispex::ATOM(s);
    }
}
impl From<Vec<Lispex>> for Lispex {
    fn from(s:Vec<Lispex>)->Lispex{
        return Lispex::LIST(s);
    }
}
*/

impl Lispex{
    fn remove(&mut self,n:usize){
    if let  Lispex::LIST(vec) = self{
        vec.remove(n);
    }else {
        panic!("cant remove from a non-List")
    }
    }
}




#[test]
fn it_works() {
    assert_eq!(Lispex::NUMBER(4), lsp![(+ 2 2)]);
  }
