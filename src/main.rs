
struct Tensor<'a> {
    dims: &'a [usize],
    array: &'a [f32],
}

// Print Functions
fn print_tensor_breaks(index: usize, dims: &[usize]) {

    let mut mod_break: usize;

    for j in 0..dims.len() {
        mod_break = 1;
        for k in j..dims.len() {
            mod_break = mod_break * dims[k];
        }
        if index % mod_break == 0 {
            print!("\n - ");
        }
    }
}


fn print_tensor(tensor: Tensor) {
    for i in 0..tensor.array.len() {
        print_tensor_breaks(i, tensor.dims);
        print!("{}", tensor.array[i]);
    }
}


// LinAlg
fn matmul(x: Tensor, y: Tensor, z: Tensor) {
    for xc in 0..x.dims[0] {
        for xr in 0..x.dims[1] {
            for yc in 0..y.dims[0] {
                z.array[xc*2+yc] = z.array[xc*2+yc] + x.array[xc*3+xr] * y.array[yc*2+xc];
            }
        }
    }
}

fn main() {

    let mut x = 5;
    println!("\nThe value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x_array: [f32; 2*3] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let x_dims: [usize; 2] = [2, 3];
    let x_tensor: Tensor = Tensor{dims: &x_dims, array: &x_array};

    let y_array: [f32; 3*2] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let y_dims: [usize; 2] = [3, 2];
    let y_tensor: Tensor = Tensor{dims: &y_dims, array: &y_array};

    let mut z_array: [f32; 2*2] = [0.0; 2*2];
    let z_dims: [usize; 2] = [2, 2];
    let z_tensor: Tensor = Tensor{dims: &z_dims, array: &z_array};

    // print_tensor(x_tensor);
    // print_tensor(y_tensor);
    // print_tensor(z_tensor);

    matmul(x_tensor, y_tensor, z_tensor)

    // print_tensor(x_tensor);
    // print_tensor(y_tensor);
    // print_tensor(z_tensor);
}
