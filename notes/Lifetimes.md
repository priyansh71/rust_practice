- If we want to return a reference from a function then it cannot be something that has been defined inside the scope of the function because that value would be dropped as soon as the function ends.
- Hence we always have to return a reference that is related to the parameters of the function 
along with lifetime annotations to specify the lifetime of the return type wherever it's called, otherwise we should return an owner which can transfrt ownership of the value itself.
- If we do return a reference from a function which has multiple borrowed parameters, we need to define the lifetime of the return type be relating it to the lifetime of the parameters, as the params' lifetimes depend how they were defined when function is called.
- This is called ```General Lifetime Annotations``` and sets the lifetime of the return value as the minimum of it's parameters lifetimes.
- This allows the Rust borrow checker to see if the references are valid at the place where the function was called.