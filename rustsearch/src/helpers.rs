use std::error::Error;
use std::fs;

pub struct Config {
    pub file_path: String,
    pub indexno: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();
        let indexno = args[2].clone();

        Ok(Config { file_path, indexno })
    }
}


pub fn read_file_to_string(file_path: &String) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}






// pub fn read_file_filtered(file_path: &String) -> Vec<&str> {
//     // let whitelist: Vec<&str> = vec!["A.B.","abbr.","Acad.","A.D.","alt.","A.M.","Assn.","Aug.","Ave.","b.","B.A.","B.C.","b.p.","B.S","c.","Capt.","cent.","co.","Col.","Comdr.","Corp.","Cpl.","d.","D.C.","Dec.","dept.","dist.","div.","Dr.","ed.","est.","al.","Feb.","fl.","gal.","Gen.","Gov.","grad.","Hon.","i.e.","in.","inc.","inc.","Inst.","Jan.","Jr.","lat.","Lib.","long.","Lt.","Ltd.","M.D.","Mr.","Mrs.","mt.","mts.","Mus.","no.","Nov.","Oct.","ph.d.","pl.","pop.","pseud.","pt.","pub.","Rev.","rev.","R.N.","Sept.","Ser.","Sgt.","Sr.","St.","uninc.","Univ.","U.S.","vol.","vs.","wt."];
//     // let blacklist: Vec<_> = vec!['(',')','[',']','{','}',',',';',':','-','/','=','?','!','*','&',' ',];

//     let re = Regex::new(r"\. |\.\n|[\[\]\{\}\\\n\-() ,;:/=?!*&]").unwrap();

//     let filecontents = read_file_to_string(file_path).expect("Should read file");

//     let x: Vec<&str> = re.split(&filecontents).collect();
//     x
// }

