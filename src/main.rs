fn main() {
    // print!("Hello, world!");
    // println!("badriana")
}

#[test]
fn variable() {
    let nama = "badriana"; //let adalah default nya immutable variable yang tidak bisa di ubah 
    // nama = "dede"; //cannot assign twice to immutable variable `nama`
    println!("{}", nama);
}

#[test]
fn variable_mut() {
    let mut nama = "dede badriana";
    println!("{}", nama);

    nama = "adrian dirgantara";
    println!("{}", nama)
}

//static typing
// rust adalah bahasa yang static typing,
//artinya setiap kita membuat variable dengan jenis data maka dia tidak akan bia di ubah ke type data lain nya
// contoh
// let mut number = 10
// number = "hello"

#[test]
fn static_typing() {
    let name = "Badriana ganteng";
    println!("{}", name);
    // name = 10; //33 |     let mut name = "Badriana ganteng";
    //    |                    ------------------ expected due to this value
    // 34 |     println!("{}", name);
    // 35 |     name = 10;
    //    |            ^^ expected `&str`, found integer
    println!("{}", name);
}

//shadowing
// shadowing adalah menduplicat sebuah variable dengan nilai yang sama
#[test]
fn shadowing() {
    let nama = 10;
    let a = "Badriana";
    println!("{}", nama);
    println!("{}", a);

    let nama = 20;
    let a = 1000;
    println!("{}", nama);
    println!("{}", a);
    // 10
    // Badriana
    // 20
    // 1000
}

/*
type data
setiap nilai di rust memilki tipe data. secara garis besar rust membagu menjadi 2 type data
1. scalar types
    representasi nilai tunggal
    integer (bilangan bulat)
    float  (bilangan deaimal)
    boolean (true false)
    character  (huruf / karakter)
2. compound types
    representasi nilai ganda
    tuple  (nilai yang sudah pasti)
    array  (nilai yang bisa diubah )
*/

#[test]
fn explicit() {
    let a: i8 = 10;
    let b: i16 = 10;
    let c: i32 = 10;
    let d: i64 = 10;
    let e: i128 = 10;
    let f: usize = 10;
    let g: u8 = 10;
    let h: u16 = 10;
    let i: u32 = 10;
    let j: u64 = 10;
    let k: u128 = 10;
    let l: usize = 10;

    let m: f32 = 10.0;
    let n: f32 = 10.2;
    let o: f64 = 11.1;
    let p: bool = true;
    let q: char = 'a';
    let r: &str = "badriana";

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
    println!("{}", f);
    println!("{}", e);
    println!("{}", g);
    println!("{}", h);
    println!("{}", i);
    println!("{}", j);
    println!("{}", k);
    println!("{}", l);
    println!("{}", m);
    println!("{}", n);
    println!("{}", o);
    println!("{}", p);
    println!("{}", q);
    println!("{}", r);
}

// integer type
/*
panjang                             negatif                     positif
8  bits                                 i8                        u8
16 bits                                 i16                       u16
32 bits                                 i32                       u32
64 bits                                 i64                       u64
128 bits                                i128                      u128
size of pointer                         isize                     usize

8-Bit
 i8: -128 hingga 127
 u8: 0 hingga 225

 16-Bit
 i16: -32,768 hingga 32,767
 u16: 0 hingga 65,535

32-Bit
i32: -2.147.483.648 hingga 2.147.483.647 (sekitar -2,1 miliar s.d 2,1 miliar)
u32: 0 hingga  4.294.967.295 (sekitar 4,2 miliar)

64-Bit
i64: -9.223.372.036.854.775.808 hingga 9.223.372.036.854.775.807 (sekitar -9,2 kuintiliun)
u64: 0 hingga 18.446.744.073.709.551.615 (sekitar 18,4 kuintiliun)


128-Bit
• i128: 170.141.183.460.469.231.731.687.303.715.884.105.728 hingga 170.141.183.460.469.231.731.687.303.715.884.105.727
• u128: 0 hingga 340.282.366.920.938.463.463.374.607.431.768.211.455
*/

#[test]
fn number() {
    let a: i8 = 20;
    let b: f32 = 10.0;

    println!("{}", a);
    println!("{}", b);
}

#[test]
fn conversi_number() {
    let a: i8 = 10;
    let b: i16 = a as i16;

    let c: i32 = b as i32;
    let d: i64 = c as i64;
    let e: i128 = d as i128;
    //ini yang tidak boleh di lakukan mengonversi dari type integer besar ke integer kecil
    let x: i64 = 1000000000;
    let y: i8 = x as i8;

    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
    println!("{}", e);
    println!("{}", y); //0 karena tidak bisa di tampung ke wadah yang lebih kecil
}

#[test]
fn numeric_operatot() {
    let a = 10;
    let b = 10;
    let c = a * b;
    let d = a - b;
    let e = a / b;
    let f = a + b;
    let g = a % b;
    println!("{}", c);
    println!("{}", d);
    println!("{}", e);
    println!("{}", f);
    println!("{}", g);
}

#[test]
fn augmented_assigments() {
    let mut a = 10;
    let b = 5;

    a += b;
    println!("{}", a);

    a -= b;
    println!("{}", a);

    a *= b;
    println!("{}", a)
}

#[test]
fn bolean_operator() {
    let absen: i8 = 70;
    let nilai_akhir: i8 = 70;
    let lulus_absen: bool = absen >= 70;
    let lulus_nilai_akhir: bool = nilai_akhir >= 70;

    let nilai_final: bool = lulus_absen && lulus_nilai_akhir;
    println!("{}", nilai_final)
}

// char  adalah tipe data karakter dan di buat menggunakan petik datu
#[test]
fn char() {
    let char: char = 'a';
    let char2: char = 'b';
    println!("{} {}", char, char2)
}

// tuple
// adalah typedata kumpulan lebih dari satu
//jumlah data tuple sudah finel artinya tidak bisa berkurang atau bertambah
//jika membuat tuple dengan total 3 maka data tidak akan bisa di ubah lagi
#[test]
fn tople() {
    let mut data: (i8, f32, bool, &str) = (20, 10.2, true, "badri");
    println!("{:?}", data);

    data.3 = "dede";
    println!("{}", data.3);
    println!("{:?}", data)
}
#[test]
fn destructuring_tuple() {
    let data: (i8, f32, bool, &str) = (1, 2.2, true, "dede");
    // println!("{:?}", data);

    let (number, _, is_verify, name) = data;
    println!("{} {} {}", number, is_verify, name)
}
#[test]
fn array() {
    //explisit
    let arr: [i8; 5] = [1, 2, 3, 4, 5];
    // no explisit
    let buah = ["jeruk", "mangga", "anggur", "terong"];
    // mut
    let mut buah2 = ["jeruk", "mangga", "anggur", "terong"];

    println!("{:?}", arr);
    println!("{:?}", buah);
    println!("{:?}", buah[2]);

    buah2[2] = "Badriana";
    println!("{:?}", buah2.join(" - "));
    println!("panjang {:?}", buah.len())
    // println!("{:?}", buah2.concat());
}

// two dimensional array (array dua dimensi). kita bisa menggunakan tipe apapun dalam array, bahkan di dalam nya type array
#[test]
fn dimendional_array() {
    // explisit
    let matrix: [[i8; 2]; 2] = [[1, 2], [3, 4]];
    println!("{:?}", matrix);
    // no explisit
    let buah = [["mangga", "jeruk"], ["anggur", "leci"], ["bambu", "kayu"]];

    println!("{:?}", buah);

    println!("{}", buah[0][0]);
    println!("{}", buah[0][1]);
    println!("{}", buah[1][0]);
    println!("{}", buah[1][1]);
    println!("{}", buah[2][1]);
}

fn function_a() {
    let a = 10;
    let b = String::from("badriana");
    println!("{} {}", a, b)
}
fn function_b() {
    let a = 10;
    let b = String::from("badriana");
    println!("{} {}", a, b)
}
#[test]
fn stack_heep() {
    function_a();
    function_b()
}

#[test]

// immmutable str
// &str adalah ukuran nya fixsize tidak bisa di ubah
//ketika variable mutable yang terjadi adalah mengganti isi variable
//namun perlu di perhatikan beberapa method &str akan mengembalikan bentuk data strig, bukan &str
//https://doc.rust-lang.org/stable/std/primitive.str.html
fn string() {
    let name: &str = " Badriana ganteng ";
    let trim: &str = name.trim();

    println!("{}", trim)
}

#[test]
fn string_type() {
    let mut name: String = String::from("dede badriana");
    name.push_str(" dirgantara");

    println!("{}", name);

    let budi = name.replace("dede", "izzah");
    println!("{}", budi)
}

#[test]
fn string_management() {
    let name = "Badriana"; // ini di simpan di stack katena type fix
    print!("{}", name);

    let nama: String = String::from("Badriana"); // ini di simpan di heap karana datanya bisa berkembang atau bertambah maupun sebaliknya
    print!("{}", nama);
}

//ounership
//setiap value di rust harus punya ownershift(variablme pemilik value)
//dalam satu wakth hanua boleh ada satu owner
//ketika owner keluar scope value akan di hapus
#[test]
fn ownership_rules() {
    //a tidak bisa di akses, belum di deklarasikan
    let a = 10; //s sudah bisa di akses

    {
        // b tidak bisa di akses, belum di deklarasikan
        let b = 10; // b sudah bisa di akses
        println!("{}", b) //b di akses
    } // scope b selesai, b di hapus dan tidak bisa di akses lagi
    println!("{}", a) //a di akses
} // scope a selesai, a dihapus dan tidak bisa di akses lagi

//data copy
// sesuai aturan di ownership rules, setiap value harus dimiliki oleh satu owner pada satu waktu
//ketika kita berinteraksi dengan data, maka data akan dimiliki oleh satu owner
// semua data bersifat fixed size(yang di simpan di stack) ketika kita tambahkan ke variable berbeda(owner baru)
//maka hasilnya data akan di copy, sehingga variable baru(owner baru) akan memiliki data hasil copy dari variable lama(owner lama)
//oleh kerena itu setiao data akan selalu di miliki oleh satu owner pada satu waktu
#[test]
fn data_copy() {
    let a = 10;
    let b = a; //copy dari a ke b
    // yang terjadi adalah b membuat data baru yang mengikuti data a
    println!("{} {}", a, b)
}

// loop
// untuk menghentikan perulangan kita bisa menggunakan break
// selain break ada namanya continue untuk menghentikan perulangan saat ini dan langsung di lanjutkan ke perulangan berikutnya

#[test]
fn loop_expresion() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 10 {
            break;
        }
        println!("continue {}", counter)
    }
}

//mengembalikan nilai menggunakan break. sama seperti return
#[test]
fn loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };
    println!("{:?}", result)

}


// loop label untuk loop dalam loop
#[test]
fn loop_label() {
    let mut number = 1;
    'nama: loop {
        let mut i =1; 
        loop {
            if number > 10 {
                break 'nama;
            }
            println!("{} x {} = {}", number, i, number*1);
            i+=1;
            if i > 10 {
                break;
            }
        }
        number += 1
    }
}

#[test]
// berbeda dengan id dan loop while dijalankan selama dia true dan melakukan lebih dari satu eksekusi 
//jika if hanya sekali jika while berkali kali
fn while_loop() {
    let mut angka = 0;
    while angka < 10 {
        if angka % 2 == 0 {
            println!("{}", angka);
            
        }
        angka += 1
    }
}