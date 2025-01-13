use rand::Rng;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Coordinate {
    name: String,
    row: i32,
    col: i32,
}

fn main() {
    let mut sector_list = Vec::new();
    gen_star_list(&mut sector_list);

    sector_list.sort_unstable_by_key(|item| (item.row, item.col));
    // This function doesn't work yet
    //print_sector(sector_list)

   for c in sector_list
   {
       println!["{:?}",c];
   }
}

fn print_sector(sl: Vec<Coordinate>)
{
let mut hex_map = vec!["  _____         _____         _____         _____              ",
                       " /01.01\\       /01.03\\       /01.05\\       /01.07\\         ",
                       "/       \\_____/       \\_____/       \\_____/       \\_____   ",
                       "\\       /01.02\\       /01.04\\       /01.06\\       /01.08\\ ",
                       " \\_____/       \\_____/       \\_____/       \\_____/       \\",
                       " /02.01\\       /02.03\\       /02.05\\       /02.07\\       / ",
                       "/       \\_____/       \\_____/       \\_____/       \\_____/  ",
                       "\\       /02.02\\       /02.04\\       /02.06\\       /02.08\\ ",
                       " \\_____/       \\_____/       \\_____/       \\_____/       \\",
                       " /03.01\\       /03.03\\       /03.05\\       /03.07\\       / ",
                       "/       \\_____/       \\_____/       \\_____/       \\_____/  ",
                       "\\       /03.02\\       /03.04\\       /03.06\\       /03.08\\ ",
                       " \\_____/       \\_____/       \\_____/       \\_____/       \\",
                       " /04.01\\       /04.03\\       /04.05\\       /04.07\\       / ",
                       "/       \\_____/       \\_____/       \\_____/       \\_____/  ",
                       "\\       /04.02\\       /04.04\\       /04.06\\       /04.08\\ ",
                       " \\_____/       \\_____/       \\_____/       \\_____/       \\",
                       " /05.01\\       /05.03\\       /05.05\\       /05.07\\       / ",
                       "/       \\_____/       \\_____/       \\_____/       \\_____/  ",
                       "\\       /05.02\\       /05.04\\       /05.06\\       /05.08\\ ",
                       " \\_____/       \\_____/       \\_____/       \\_____/       \\",
                       " /06.01\\       /06.03\\       /06.05\\       /06.07\\       / ",
                       "/       \\_____/       \\_____/       \\_____/       \\_____/  ",
                       "\\       /06.02\\       /06.04\\       /06.06\\       /06.08\\ ",
                       " \\_____/       \\_____/       \\_____/       \\_____/       \\",
                       " /07.01\\       /07.03\\       /07.05\\       /07.07\\       / ",
                       "/       \\_____/       \\_____/       \\_____/       \\_____/  ",
                       "\\       /07.02\\       /07.04\\       /07.06\\       /07.08\\ ",
                       " \\_____/       \\_____/       \\_____/       \\_____/       \\",
                       " /08.01\\       /08.03\\       /08.05\\       /08.07\\       / ",
                       "/       \\_____/       \\_____/       \\_____/       \\_____/  ",
                       "\\       /08.02\\       /08.04\\       /08.06\\       /08.08\\ ",
                       " \\_____/       \\_____/       \\_____/       \\_____/       \\",
                       " /09.01\\       /09.03\\       /09.05\\       /09.07\\       / ",
                       "/       \\_____/       \\_____/       \\_____/       \\_____/  ",
                       "\\       /09.02\\       /09.04\\       /09.06\\       /09.08\\ ",
                       " \\_____/       \\_____/       \\_____/       \\_____/       \\",
                       " /10.01\\       /10.03\\       /10.05\\       /10.07\\       / ",
                       "/       \\_____/       \\_____/       \\_____/       \\_____/  ",
                       "\\       /10.02\\       /10.04\\       /10.06\\       /10.08\\ ",
                       " \\_____/       \\_____/       \\_____/       \\_____/       \\",
                       "       \\       /     \\       /     \\       /     \\       / ",
                       "        \\_____/       \\_____/       \\_____/       \\_____/  "];

    // Add stars to map
    /*for star in sl
    {   
        // TODO: figure out how string replacement works. I think the way to go is slicing
        let starRow: usize = star.row.try_into().unwrap();
        let starCol: usize = star.col.try_into().unwrap();
        let mut rowString = hex_map[starRow];

        let result = str::replace("Hello World!", "!", "?");
        
        rowString[starCol] = "!"; // "!!!!!!!";
    }*/

    // Print vector
    for line in hex_map
    {
        println!("{}", line);
    }
}

fn gen_star_list(sl: &mut Vec<Coordinate>) {
    // The standard sector map is a grid of hexagons 8 wide by
    // 10 high, with hex templates easily acquired from the net.
    // Roll 1d10 and add 20 to determine the total number of
    // stars in the sector, or simply choose a number that suits.
    // For the first twenty or so stars, roll 1d8 and 1d10
    // together to determine the column and row in which
    // to place the star. If the hex is already occupied, place it
    // adjacent in the direction of the nearest edge.
    // Some columns have “half-hexes” in them, and only
    // 9 full hexes; if you roll a 10 for such a column, either
    // reroll or place it at the bottom-most full hex.
    
    let star_count = rand::thread_rng().gen_range(1..11) + 20;
    // println!("Starcount: {}",star_count);

    let mut i = 0;
    while i < star_count {
        let row = rand::thread_rng().gen_range(1..11);
        let col = rand::thread_rng().gen_range(1..9);

        // Generate star name
        let star_name = generate_name();

        // Roll coordinates
        let coord = Coordinate{name: star_name.clone(), row: row, col: col};
        if sl.len() == 0
        {
            sl.push(coord);
            i += 1;
            continue;
        }
        if !sl.contains(&coord)
        {
            sl.push(coord);
            // println!("{} {} added straight iter {}", row, col, i);
            i += 1;
            continue;
        }
        else
        {
            let row_plus = Coordinate{name: star_name.clone(), row: row+1, col: col};
            let col_plus = Coordinate{name: star_name.clone(),row: row, col: col+1};

            if !sl.contains(&row_plus)
            {
                sl.push(coord);
                // println!("{} {} added row_plus iter {} ", row+1, col, i);
                i += 1;
                continue;
            }
            if !sl.contains(&col_plus)
            {
                sl.push(coord);
                // println!("{} {} added col_plus iter {}", row, col+1, i);
                i += 1;
                continue;
            }
        }
        i += 1;
    }
}

fn generate_name() -> String 
{
   let nm1 = vec!["a","e","i","o","u","","","","","","","","","","","","","",""];
   let nm2 = vec!["b","c","d","f","g","h","j","k","l","m","n","p","q","r","s","t","v","w","x","y","z","br","cr","dr","gr","kr","pr","sr","tr","str","vr","zr","bl","cl","fl","gl","kl","pl","sl","vl","zl","ch","sh","ph","th"];
   let nm3 = vec!["a","e","i","o","u","a","e","i","o","u","a","e","i","o","u","ae","ai","ao","au","aa","ea","ei","eo","eu","ee","ia","io","iu","oa","oi","oo","ua","ue"];
   let nm4 = vec!["b","c","d","f","g","h","j","k","l","m","n","p","q","r","s","t","v","w","x","y","z","br","cr","dr","gr","kr","pr","sr","tr","str","vr","zr","bl","cl","fl","hl","gl","kl","ml","nl","pl","sl","tl","vl","zl","ch","sh","ph","th","bd","cd","gd","kd","ld","md","nd","pd","rd","sd","zd","bs","cs","ds","gs","ks","ls","ms","ns","ps","rs","ts","ct","gt","lt","nt","st","rt","zt","bb","cc","dd","gg","kk","ll","mm","nn","pp","rr","ss","tt","zz"];
   let nm5 = vec!["","","","","","","","","","","","","","b","c","d","f","g","h","k","l","m","n","p","r","s","t","x","y","b","c","d","f","g","h","k","l","m","n","p","r","s","t","x","y","cs","ks","ls","ms","ns","ps","rs","ts","ys","ct","ft","kt","lt","nt","ph","sh","th"];
    let mut ret = String::new();
    // flip coin
    let flip = rand::thread_rng().gen_range(0..2);
    // if flip == 0
    if flip == 0
    {  
       let rnd  = rand::thread_rng().gen_range(0..nm1.len());
       let rnd2 = rand::thread_rng().gen_range(0..nm2.len());
       let rnd3 = rand::thread_rng().gen_range(0..nm3.len());
       let rnd6 = rand::thread_rng().gen_range(0..nm5.len());
       ret = nm1[rnd].to_owned()+nm2[rnd2]+nm3[rnd3]+nm5[rnd6];
    }
    // if flip == 1
    else if flip == 1
    {
        let rnd  = rand::thread_rng().gen_range(0..nm1.len());
        let rnd2 = rand::thread_rng().gen_range(0..nm2.len());
        let rnd3 = rand::thread_rng().gen_range(0..nm3.len());
        let rnd4 = rand::thread_rng().gen_range(0..nm4.len());
        let mut rnd5 = rand::thread_rng().gen_range(0..nm3.len());
        let rnd6 = rand::thread_rng().gen_range(0..nm5.len());
        if rnd3>14
        {
            while rnd5>14
            {
                rnd5 = rand::thread_rng().gen_range(0..nm3.len());
            }
        }
        ret = nm1[rnd].to_owned()+nm2[rnd2]+nm3[rnd3]+nm4[rnd4]+nm3[rnd5]+nm5[rnd6];
    }
    return ret;
}
