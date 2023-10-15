use ldap3::*;
use std::process::exit;
use ldap3::result::Result;


fn main() {
    
    let mut ldap = LdapConn::new("ldap://127.0.0.1:3268");
    let mut ldapcon = match ldap{
        Ok(l) => l,
        Err(r) => panic!("{}", r),
    };

    ldapcon.simple_bind("CN=Administrator,CN=Users,DC=Stenaniuk,DC=local", "").unwrap();
    let res = ldapcon.search("DC=Stenaniuk,DC=local",Scope::Subtree,"(objectclass=user)", vec!["dn"]).unwrap();
    let (re, ldapresult) = res.success().unwrap();

    for i in re{
        println!("{:#?}", SearchEntry::construct(i));
    }

    
}
