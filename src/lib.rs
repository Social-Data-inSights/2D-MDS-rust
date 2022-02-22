use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

/* #[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
} */

#[wasm_bindgen]
pub fn mdsRustF(dissimalirities_js: &[f64], init_js: &[f64], n_sample: usize, n_components: u32, max_iter: u32, eps: f64 )
    -> Vec<f64> {
    let mut disparities: Vec<Vec<f64>> = vec![vec![0.0; n_sample]; n_sample];
    for i in 0..n_sample {
        for j in 0..n_sample {
            disparities[i][j] = dissimalirities_js[i * n_sample + j]
        }
    }

    let mut X: Vec<[f64;2]> = vec![[0.0; 2]; n_sample];
    for i in 0..n_sample {
        for j in 0..2 {
            X[i][j] = init_js[i * 2 + j]
        }
    }

    let mut old_stress: f64 = -1.0;
    let mut stress: f64;

    /* if n_sample != init.len() {
        let msg = &format!("init matrix should be of shape ({}, {})", n_sample, n_components);
        throw!(msg);
    } */


    // as we don't read any data from ratio, there is no need to initialize it at every iteration
    let mut ratio: Vec<Vec<f64>> = vec![vec![0.0; n_sample]; n_sample];

    for it in 0..max_iter {
        stress = 0.;
        for (i, ([x1, y1], i_disparities)) in X.iter().zip(disparities.iter()).enumerate() {
            for (j, ([x2, y2], ij_disparities)) in X.iter().zip(i_disparities.iter()).enumerate().take(i) {
                let ij_dis: f64 = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
                
                // Compute stress
                stress += (ij_dis - ij_disparities).powi(2);

                let rs: f64 = ij_disparities / (if ij_dis == 0f64 { 1e-5f64 } else { ij_dis });
                ratio[i][j] = rs;
                ratio[j][i] = rs;
            }
        }
        stress /= 2.; 
        
        for i in 0..n_sample {
            let sum: f64 = ratio[i].iter().sum() ;
            ratio[i][i] -= sum;
        }

        for i in 0..2 {
            let i_x: &Vec<f64> = &(X.iter().map(|x| x[i]).collect());
            for (j, j_ratio) in ratio.iter().enumerate() {
                let new_x = i_x.iter().zip(j_ratio.iter()).fold(0f64, |sum, (ij_x,  ij_ratio)| sum + ij_ratio * ij_x );
                X[j][i] = - (new_x / (n_sample as f64)); 
            }
        }

        let mut end_dis_score: f64 = 0.0;
        for x in &X {
            let mut x_sum: f64 = 0.0;
            for y in x {
                x_sum += y.powi(2);
            }
            end_dis_score += x_sum.sqrt();
        }
        if old_stress != -1.0 {
            if (old_stress - stress / end_dis_score) < eps {
                break
            }
        }
        old_stress = stress / end_dis_score;
        
    }
    let mut res_x: Vec<f64> = vec![0.0; n_sample * 2] ;
    for i in 0..n_sample {
        for j in 0..2 {
            res_x[i * 2 + j] = X[i][j];
        }
    }

    return res_x
}
