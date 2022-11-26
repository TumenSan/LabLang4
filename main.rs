use std::io::Write;
use std::env;
use std::fs;
use std::io::Read;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
struct pt {
    x: f32,
    y: f32
}

#[derive(Debug, Clone, Copy)]
struct line {
    a: f32,
    b: f32,
    c: f32
}

fn det(a: f32, b: f32, c: f32, d: f32) -> f32{
    return a * d - b * c;
}

fn intersect(m: line, n: line, mut res: pt) -> (f32, f32, bool){
    let mut zn = det(m.a, m.b, n.a, n.b);
    zn = zn.abs();
    let mut bool1 = true;
    if zn < 0.0001{
        bool1 = false;
    }
    res.x = (-1.0 as f32 * det(m.c, m.b, n.c, n.b) as f32) / zn as f32;
    res.y = (-1.0 as f32 * det(m.a, m.c, n.a, n.c) as f32) / zn as f32;
    return(res.x, res.y, bool1);
}

fn abcAnswer(x1Luch: f32, y1Luch: f32, x2Luch: f32, y2Luch: f32) -> (f32, f32, f32){
    let mut l = line {
        a: 0.0,
        b: 0.0,
        c: 0.0,
    };
    l.b = x2Luch as f32 - x1Luch as f32;
    l.a = -1.0 as f32 * (y2Luch as f32 - y1Luch as f32);
    l.c = -1.0 as f32 * (l.a as f32 * x1Luch as f32 + l.b as f32 * y1Luch as f32);
    return(l.a, l.b, l.c);
}

// Для обработки ошибок, возвращаемое значение оборачивается в Result
// Возвращаем `Iterator` для построчного чтения файла.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut MasLines = Vec::new();

    if let Ok(lines) = read_lines("./text.txt") {
        // Получает итератор, который возвращает Option
        for line in lines {
            if let Ok(ip) = line {
                MasLines.push(ip);//
                //println!("{}", ip);
            }      
        }   
    }
    //println!("{} {}", MasLines[0], MasLines[1]);

    let mut pt1 = pt {
        x: 0.0,
        y: 0.0,
    };

    let mut line1 = line {
        a: 0.0,
        b: 0.0,
        c: 0.0,
    };

    let mut line2 = line {
        a: 0.0,
        b: 0.0,
        c: 0.0,
    };

    let mut ln = Vec::new();

    let mut lenVec = MasLines.len();
    let mut str1;

    for i in 0..lenVec{
        str1 = MasLines[i].clone();
        str1 = str1.to_string();
        str1.trim();
        str1 = str1.replacen(" ", ",", 1);
        let v: Vec<&str> = str1.split_terminator(',').collect();
        for j in 0..4{
            let mut xi: f32 = v[j].parse().unwrap();
            ln.push(xi as f32);
            //println!("num {}", xi as f32);
        }
        //println!("otrezok {}", str1);
    }


    let mut dist = f32::MAX;
    let mut numberLine = 1;
    let mut dist2 = -1.0_f32;

    for i in 0..lenVec - 1{
        (line1.a, line1.b, line1.c) = abcAnswer(ln[0], ln[1], ln[2], ln[3]);
        (line2.a, line2.b, line2.c) = abcAnswer(ln[4 + i * 4], ln[5 + i * 4], ln[6 + i * 4], ln[7 + i * 4]);
        let mut answer = intersect(line1, line2, pt1);
        //println!("{} {} {}", answer.0, answer.1, answer.2);

        if (answer.2 && 
        (((answer.0 >= ln[4 + i * 4] && answer.0 <= ln[6 + i * 4]) || 
        (answer.0 <= ln[4 + i * 4] && answer.0 >= ln[6 + i * 4])) && 
        ((answer.1 >= ln[5 + i * 4] && answer.1 <= ln[7 + i * 4]) || 
        (answer.1 <= ln[5 + i * 4] && answer.1 >= ln[7 + i * 4])))) { //otrezok and lines
            //println!("otrezok ok {}", 1);
            if ((answer.0 > ln[0]) == (ln[2] > ln[0]) && (answer.1 > ln[1]) == (ln[3] > ln[1])) { //Luch
                //println!("luch ok {}", 1);
                dist2 = ((answer.0 - ln[0]) * (answer.0 - ln[0]) + (answer.1 - ln[1]) * (answer.1 - ln[1])).sqrt();
                //println!("{}", dist2);
                if dist2 < dist{
                    dist = dist2;
                    numberLine = i;
                }
            }
            else {
                //println!("luch not ok {}", 0);
            }
        }
        else {
            //println!("otrezok not ok {}", 0);
        }
        
    }
    if dist2 > 0.0{
        //println!("{} {} {} {} {}", dist, ln[(numberLine + 1) * 4], ln[(numberLine + 1) * 4 + 1], ln[(numberLine + 1) * 4 + 2], ln[(numberLine + 1) * 4 + 3]);
        println!("{} {} {} {}", ln[(numberLine + 1) * 4], ln[(numberLine + 1) * 4 + 1], ln[(numberLine + 1) * 4 + 2], ln[(numberLine + 1) * 4 + 3]);
    }
    
}

//rustc main.rs
//./main