use reqwest;

pub struct Check{
    pub version:String,
    pub resume:String,
    pub date:String,
    pub ws:String
}
impl Default for Check{
    fn default()->Check{
        return Check{
            version:String::new(),
            resume:String::new(),
            date:String::new(),
            ws:String::new()
        }
    }
}

pub async fn req_eve()->Result<Check,String>{
    let checker = reqwest::get("https://www.eveonline.com/fr/news");
    match checker.await{
        Ok(x)=>{
            let body = x.text().await.unwrap();
            let text = body.split('>');
            let mut read = false;
            let mut url = String::new();
            for l in text{  // Requête sur main page
                if !read{
                    if l.contains("<div class=\"PatchNotesLatest_patchNotes"){
                        read = true;
                    }
                }
                else{
                    let mut write = false;
                    for c in l.chars(){
                        if !write{
                            if c == '"'{
                                write = true;
                            }
                        }
                        else{
                            if c != '"'{
                                url.push(c);
                            }
                            else{
                                break
                            }
                        }
                    }
                    read = false;
                }   
            }
            url = String::from("http://www.eveonline.com")+&url;
            let m_info = reqwest::get(url);
            match m_info.await{ // Requête sur page patch
                Ok(y)=>{
                    let ybody = y.text().await.unwrap();
                    let ytext = ybody.split(">");
                    let mut read = 0;
                    let mut check = Check::default();
                    for l in ytext{
                        match read{
                            0=>{
                                if l.contains("<h1"){
                                    read = 1;
                                }
                                if l.contains("<span class=\"DateAndAuthor"){
                                    read = 2;
                                }
                                if l.contains("<ul class=\"ListItem"){
                                    read = 3;
                                }
                            }
                            1=>{ // Récupération de la version
                                for c in l.chars(){
                                    if c != '>' && c!='<'{
                                        check.version.push(c);
                                    }
                                    else if c == '<'{
                                        break
                                    }
                                }
                                let clean = check.version.split(" ");
                                check.version = clean.last().unwrap().to_string();                                
                                read = 0;
                            },
                            2=>{ // Récupération de la date
                                let mut word = String::new();
                                for c in l.chars(){
                                    if c != '"' && c != '<'{
                                        word.push(c);
                                    }
                                    else if c == '<'{
                                        break
                                    }
                                }
                                check.date = word;
                                read = 0;
                            },
                            3=>{ // Récupération du contenu
                                if !l.starts_with('<'){
                                    if l.contains("General"){
                                        read = 0
                                    }
                                    else{
                                        for e in l.split("</p"){
                                            check.resume = e.to_string();
                                            break
                                        }
                                        break
                                    }
                                }
                            }
                            _=>{}   
                        }
                    }
                    check.ws = String::from("https://www.eveonline.com");
                    return Ok(check)
                },
                Err(err)=>{
                    return Err(err.to_string())
                }
            }
        },
        Err(err)=>{
            return Err(err.to_string())
        }
    }
}

pub async fn req_dreadcast()->Result<Check,String>{
    let checker = reqwest::get("http://www.dreadcast.net");
    match checker.await{
        Ok(x)=>{
            let body = x.text().await.unwrap();
            let text = body.lines();
            let mut read = false;
            let mut extract = true;
            let mut content = Check::default();
            for l in text{
                if !read{
                    if l.contains("div id=\"actu\""){
                        read = true;
                    }
                }
                else if read && extract{    // Extraction des données de la dernière News
                    if l.contains("<h3>"){
                        content.version = extract_c(l);
                    }
                    else if l.contains("<span class=\"intro\""){
                        content.resume = extract_c(l);
                    }
                    else if l.contains("<span>"){
                        content.date = extract_c(l);
                    }
                    else if l.contains("<small>"){
                        content.date.push_str(extract_c(l).as_str());
                        extract = false;
                    }  
                }
                
            }
            content.ws = String::from("http://www.dreadcast.net");
            return Ok(content);
        },
        Err(err)=>{
            return Err(err.to_string());
        }
    }
}

fn extract_c(x:&str)->String{
    let mut clean = String::new();
    let mut read = false;
    for c in x.chars(){
        if !read{
            if c == '>'{
                read = true;
            }
        }
        else{
            if c == '<'{
                read = false
            }
            else{
                clean.push(c);
            }
        }
    }
    let t = clean.trim().to_owned();
    return t
}