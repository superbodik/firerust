use fr_passwords::import::import_csv::import_csv;
use fr_passwords::import::import_password_file::import_password_file;
use fr_passwords::import::parse_csv::parse_csv;
use fr_passwords::import::password_import_error::PasswordImportError;

const CHROME: &str = "name,url,username,password,note\r\n\
example.com,https://example.com/login,ada,\"pa,ss\"\"word\",\r\n\
local,http://127.0.0.1:8080/login,bob,secret,\"two\nlines\"\r\n\
app,android://hash@com.app/,carl,x,\r\n\
empty,https://nothing.test/,dan,,\r\n";

const FIREFOX: &str = "\"url\",\"username\",\"password\",\"httpRealm\",\"formActionOrigin\",\"guid\"\n\
\"https://ff.test\",\"eve\",\"p1\",\"\",\"https://ff.test\",\"{1}\"\n";

const BITWARDEN: &str = "folder,login_uri,login_username,login_password\n\
work,https://bw.test/signin,frank,zzz\n";

#[test]
fn the_parser_handles_quotes_commas_and_line_breaks() {
    let rows = parse_csv(CHROME);

    assert_eq!(rows[1][3], "pa,ss\"word");
    assert_eq!(rows[2][4], "two\nlines");
    assert_eq!(rows.len(), 5);
}

#[test]
fn the_parser_copes_with_a_missing_final_newline_and_blank_lines() {
    let rows = parse_csv("a,b\n\n1,2");

    assert_eq!(rows, vec![vec!["a", "b"], vec!["1", "2"]]);
}

#[test]
fn a_chrome_export_gives_web_logins_only() {
    let found = import_csv(CHROME).expect("imported");

    assert_eq!(found.len(), 2);
    assert_eq!(found[0].origin, "https://example.com");
    assert_eq!(found[0].username, "ada");
    assert_eq!(found[0].password, "pa,ss\"word");
    assert_eq!(found[1].origin, "http://127.0.0.1:8080");
}

#[test]
fn a_firefox_export_is_understood() {
    let found = import_csv(FIREFOX).expect("imported");

    assert_eq!(found.len(), 1);
    assert_eq!(found[0].origin, "https://ff.test");
    assert_eq!(found[0].username, "eve");
}

#[test]
fn a_bitwarden_export_is_understood() {
    let found = import_csv(BITWARDEN).expect("imported");

    assert_eq!(found[0].origin, "https://bw.test");
    assert_eq!(found[0].password, "zzz");
}

#[test]
fn a_file_without_the_needed_columns_is_rejected() {
    assert!(import_csv("a,b\n1,2\n").is_none());
    assert!(import_csv("").is_none());
}

#[test]
fn the_file_importer_reports_missing_and_unrelated_files() {
    let dir = std::env::temp_dir().join(format!("firerust-pw-import-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("dir");
    let good = dir.join("passwords.csv");
    let bad = dir.join("notes.csv");
    std::fs::write(&good, format!("\u{feff}{CHROME}")).expect("write");
    std::fs::write(&bad, "hello\nworld\n").expect("write");

    assert_eq!(import_password_file(&good).expect("good").len(), 2);
    assert!(matches!(
        import_password_file(&bad),
        Err(PasswordImportError::UnknownFormat)
    ));
    assert!(matches!(
        import_password_file(&dir.join("none.csv")),
        Err(PasswordImportError::NotFound(_))
    ));
    let _ = std::fs::remove_dir_all(&dir);
}
