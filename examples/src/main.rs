rzhavchina::rzhavchina! {
    типок Пацик {
        возраст: Строчка,
        сдал_егэ: попонятиямли,
    }

    пацикисделают отсюда() {
        пацик мутант Пацики: Банда<Пацик> = Банда::слепить();
        втечении номер глянуть 0..10 {
            Пацики.запихнуть(Пацик {
                возраст: Строчка::украситьиз(номер.застрочить()),
                сдал_егэ: неправда
            });
        }

        втечении пац глянуть Пацики {
            допустим пац.сдал_егэ {
                накалякать!("За базар ответишь!?");
            } забазаротвечай {
                накалякать!("Молодец мужик");
            }
        }
    }
}
