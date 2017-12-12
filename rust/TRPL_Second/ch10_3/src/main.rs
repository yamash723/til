// ----------------------------------------
// Error pattern
// ----------------------------------------
// fn hoge(vec0: &Vec<i32>, vec1: &Vec<i32>) -> &Vec<i32> {
//     vec0
// }

// fn main() {
//     let vec0 = vec![0];
//     let vec1 = vec![1];
//     {
//         let mut vec2 = &vec0;
//         let mut vec3 = &vec1;
//         {
//             let vec4 = vec![2];
//             vec2 = hoge(&vec4, &vec3); // <- Error
//         } // <- ここでScopeが切れて破棄されるvec4を、まだライフタイムが先のvec2に使用したことでNGに
//         println!("{} {}", vec2, vec3);
//     }
// }

fn hoge<'a, 'b>(vec0: &'a Vec<i32>, vec1: &'b Vec<i32>) -> &'a Vec<i32> {
    vec0
}

fn main() {
    let vec0 = vec![0];
    let vec1 = vec![1];
    {
        let mut vec2 = &vec0;
        let mut vec3 = &vec1;
        {
            let vec4 = vec![2];
            vec2 = hoge(&vec3, &vec4);
        }
        println!("{} {}", vec2[0], vec3[0]);
    }
}