//

#![allow(unused)]

#[derive(Debug)]
enum InesEmployee {
    WiAssis {
        years_worked: u8,
        is_mse_student: bool,
    },
    WiMa {
        years_worked: u8,
    },
    BigShot {
        has_cool_vim_themed_mug: bool,
        wears_hat_correctly: bool,
    },
}

fn main() {
    let employees = [
        InesEmployee::WiAssis {
            years_worked: 1,
            is_mse_student: true,
        },
        InesEmployee::WiAssis {
            years_worked: 2,
            is_mse_student: false,
        },
        InesEmployee::WiMa { years_worked: 4 },
        InesEmployee::BigShot {
            has_cool_vim_themed_mug: true,
            wears_hat_correctly: true,
        },
        InesEmployee::BigShot {
            has_cool_vim_themed_mug: false,
            wears_hat_correctly: false,
        },
    ];

    println!("{:#?}", employees);
}
