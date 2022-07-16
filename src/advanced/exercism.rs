pub fn fun(){
    pub fn _reverse(input: &str) -> String {
        let mut reversed = String::new();
        for c in input.chars().rev() {
            reversed.push_str(&c.to_string());
        }
        reversed
    }

    pub fn _square_of_sum(n: u32) -> u32 {
        let mut result : u32 = n*(n+1)/2;
        result = result*result;
        result
    }
    
    pub fn _sum_of_squares(n: u32) -> u32 {
        let mut result : u32 = 1;
        let mut sum : u32 = 0;
        loop {
            if result == n+1{
                return sum;
            }
            sum += result*result;
            result+=1;
        }
    }
    
    pub fn _difference(n: u32) -> u32 {
        let a = _square_of_sum(n);
        let b = _sum_of_squares(n);
        a-b
    }

    pub fn factors(n: u64) -> Vec<u64> {
        let mut factors : Vec<u64>= Vec::new();
        let mut temp = n;
        let ceil : u64 = f64::sqrt(n as f64) as u64 + 1;
        for i in 2..ceil {
            while temp%i==0  {
                factors.push(i);
                temp/=i;
            }
        }
        if temp > 1{
            factors.push(temp);
        }
        factors
    }

    
}