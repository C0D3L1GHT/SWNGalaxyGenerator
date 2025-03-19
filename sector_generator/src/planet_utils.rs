use rand::Rng;

pub fn generate_name() -> String 
{
    return gen_name_classic();
}

fn gen_name_from_rules() ->
{
    let ipa_pulmonic_consonants: Vec<char> = vec![
        'p', 'b', 't', 'd', 'ʈ', 'ɖ', 'c', 'ɟ', 'k', 'ɡ', 'q', 'ɢ', 'ʔ',
        'm', 'ɱ', 'n', 'ɳ', 'ɲ', 'ŋ', 'ɴ',
        'ʙ', 'r', 'ʀ', 
        'ɾ', 'ɽ',
        'ɸ', 'β', 'f', 'v', 'θ', 'ð', 's', 'z', 'ʃ', 'ʒ', 'ʂ', 'ʐ', 'ç', 'ʝ', 'x', 'ɣ', 'χ', 'ʁ', 'ħ', 'ʕ', 'h', 'ɦ',
        'ɬ', 'ɮ',
        'ʋ', 'ɹ', 'ɻ', 'j', 'ɰ',
        'l', 'ɭ', 'ʎ', 'ʟ'
    ];

    let ipa_pulmonic_consonants: Vec<char> = vec![
        'ʘ', 'ǀ', 'ǃ', 'ǂ', 'ǁ',
        'ɓ', 'ɗ', 'ʄ', 'ɠ', 'ʛ'
    ];

    let ipa_vowels: Vec<char> = vec![
        'i', 'y', 'ɨ', 'ʉ', 'ɯ', 'u',
        'ɪ', 'ʏ',
        'e', 'ø', 'ɘ', 'ɵ', 'ɤ', 'o',
        'ə',
        'ɛ', 'œ', 'ɜ', 'ɞ', 'ʌ', 'ɔ',
        'æ', 'ɐ',
        'a', 'ɶ', 'ɑ', 'ɒ',
    ];

    let ipa_suprasegmentals: Vec<char> = vec![
        'ˈ', 'ˌ', 'ː', 'ˑ', '|', '‖', '.' 
    ];

    // NOTE: This is just a basic set
    let ipa_tones_and_accents: Vec<char> = vec![
        '˥', '˦', '˧', '˨', '˩',
    ];

    let ipa_diacritics: Vec<char> = vec![
        '\u{0303}', // Combining tilde (nasalization)
        '\u{0308}', // Combining diaeresis (centralization)
        '\u{030A}', // Combining ring (voiceless)
        '\u{0325}', // Combining ring below (voiceless)
        '\u{0329}', // Combining vertical line below (syllabic)
        '\u{032C}', // Combining seagull below (non-syllabic)
        '\u{0330}', // Combining tilde below (creaky voice)
        '\u{0334}', // Combining tilde overlay (velarized/pharyngealized)
        '\u{0339}', // Combining right half ring below (advanced tongue root)
        '\u{031C}', // Combining left half ring below (retracted tongue root)
        '\u{031F}', // Combining plus sign below (advanced)
        '\u{0320}', // Combining minus sign below (retracted)
        '\u{0306}', // Combining breve (extra short)
    ];

    let ipa_other_symbols: Vec<char> = vec![
        '↗', '↘', 'ʍ', 'ɥ', 'ʜ', 'ʢ', 'ʡ', 'ɕ', 'ʑ', 'ɺ', 'ɧ'
    ];


}

fn gen_name_classic() -> String
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
