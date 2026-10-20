use std::io;
#[derive(Debug)]
enum Status {
    complete,
    procces,
}

#[derive(Debug)]
struct Task {
    title: String,
    opis: String,
    status: Status,
}
fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    loop {
        println!("\n1 — добавить задачу");
        println!("2 — показать все задачи");
        println!("3 — выйти");
        println!("Выберите действие: ");
        // все, с чем взаимодействуем.

        let mut choice = String::new(); // переменная, благодаря которой осуществляется выбор.
        io::stdin().read_line(&mut choice).expect("loser");

        match choice.trim() {
            "1" => {
                println!("привет, расскажи о своей задаче.");

                println!("для начало: как ее назовем? ");

                let mut titles = String::new();
                io::stdin().read_line(&mut titles).expect("idk");
                let titles = titles.trim().to_string();

                println!("отлично, теперь в кратце опиши ее: ");

                let mut opisanie = String::new();
                io::stdin()
                    .read_line(&mut opisanie)
                    .expect("а как по другому?");
                let opisanie = opisanie.trim().to_string();

                println!("great.");

                let tusk = Task {
                    title: titles,
                    opis: opisanie,
                    status: Status::procces,
                };
                tasks.push(tusk);
            }

            "2" => {
                if tasks.is_empty() {
                    println!("задач нету.")
                } else {
                    for (i, task) in tasks.iter().enumerate() {
                        println!("{}: {:?}", i + 1, task);
                    }
                }
            }
            "3" => {
                println!("досвидания!");
                break;
            }
            &_ => {
                println!("неверный ввод");
                continue;
            }
        }
    }
}
