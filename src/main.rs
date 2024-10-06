use std::io::Write;         // подключение трейтов Read и Write из std
use std::fs::File;

const CU: [f64; 3] = [234.5, 3.45, 17.241];
const AL: [f64; 3] = [228.0, 2.5, 28.264];
const LEAD: [f64; 3] = [230.0, 1.45, 214.0];
const STEEL: [f64; 3] = [202.0, 3.8, 138.0];

const CROSS_SECTION: [f64; 15] = [16.0, 25.0, 35.0, 50.0, 70.0, 95.0, 120.0, 
        150.0, 185.0, 240.0, 300.0, 400.0, 500.0, 630.0, 800.0];

const MATERIALS: [[f64; 3]; 4] = [CU, AL, LEAD, STEEL];

fn main() {
  
    let (t1, t2) = (90.0f64, 250.0f64);
    let duration: f64 = 1.0;
    let file_id = "currents.txt";
    
    let home_dir = std::env::home_dir().unwrap().display();
    let mut ratios = vec![];
    let path = format!("{}/{}", home_dir, file_id);
    let mut file: File = File::create(path).unwrap();

    let st6 = 10f64.powf(6.0);
    let st9 = 10f64.powf(-9.0);

    let top = format!(
        "\n\nДопустимые токи КЗ, кА (t = {} сек)\t\t\t\t\tT0 = {}\u{00B0}C, T1 = {}\u{00B0}C\n{:-<94}",
        duration, t1, t2, ""
    );
    let cont = format!("\t\t\t\tМатериал жилы или экрана кабеля\nS, mm^2\t{:-<86}", "");
    let elem = format!("\t\tCu\t\t\tAl\t\t\tLead\t\t\tSteel\n{:-<94}", "");

    println!("{top}"); writeln!(file, "{top}").unwrap(); //печать и вывод в файл
    println!("{cont}"); writeln!(file, "{cont}").unwrap();
    println!("{elem}"); writeln!(file, "{elem}").unwrap();
    
 
    CROSS_SECTION.map(| s | {

        print!("{s}\t\t"); 
        write!(file, "{s}\t\t").unwrap();

        MATERIALS.map(| m | {

            let ratio = (m[1] * st6 * (m[0] + 20.0) * 10f64.powf(-12.0) /
                (m[2] * st9) * ((t2 + m[0]) / (t1 + m[0])).ln()).powf(0.5);
        
            let current = s * ratio * duration.powf(-0.5) * 10f64.powf(-3.0); 

            if s == 800.0 {
                print!("{:.2}\t\t\t", current);
                write!(file, "{:.2}\t\t\t", current).unwrap();
                ratios.push(ratio);
            } else {
                print!("{:.3}\t\t\t", current);
                write!(file, "{:.3}\t\t\t", current).unwrap();
            }
        });
        println!("");
        writeln!(file, "").unwrap();
    });
    print!("{:-<94}\nk, кА/мм^2", "");
    write!(file, "{:-<94}\nk, кА/мм^2", "").unwrap();

    for i in ratios.iter() {
        print!("\t{:.4}\t\t", i * 10f64.powf(-3.0));
        write!(file, "\t{:.4}\t\t", i * 10f64.powf(-3.0)).unwrap();
    }
    println!("\n\n");
}

