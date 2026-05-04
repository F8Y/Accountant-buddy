import s from './welcome-screen.module.css';

type WelcomeScreenProps = {
    className?: string;
};

export const WelcomeScreen = ({ className }: WelcomeScreenProps) => {
    return (
        <div className={className}>
            <div className={s['welcome-screen']}>
                <h1 className={s['title']}>
                    Buh Buddy
                </h1>
                <p className={s['subtitle']}>
                    AI-помощник для бухгалтеров с экспертизой по российскому законодательству.
                </p>
                <p className={s['subtitle']}>
                    Задавайте вопросы, получайте ответы со ссылками на НК РФ, ФСБУ и письма ФНС.
                </p>
                <ul className={s['features']}>
                    <li>Актуальная нормативная база</li>
                    <li>Цитаты со ссылками на источники</li>
                    <li>Локальная работа с данными</li>
                </ul>
            </div>
        </div>
    );
};
