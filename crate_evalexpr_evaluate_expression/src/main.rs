use evalexpr::*;

fn main() {
    assert_eq!(eval("1 + 2 + 3"), Ok(Value::from(6)));
    // `eval` returns a variant of the `Value` enum,
    // while `eval_[type]` returns the respective type directly.
    // Both can be used interchangeably.
    assert_eq!(eval_int("1 + 2 + 3"), Ok(6));
    assert_eq!(eval("1 - 2 * 3"), Ok(Value::from(-5)));
    assert_eq!(eval("1.0 + 2 * 3"), Ok(Value::from(7.0)));
    assert_eq!(eval("true && 4 > 2"), Ok(Value::from(true)));

    // assign

    let mut context = HashMapContext::new();
    // Assign 5 to a like this
    assert_eq!(
        eval_empty_with_context_mut("a = 5", &mut context),
        Ok(EMPTY_VALUE)
    );
    // The HashMapContext is type safe, so this will fail now
    assert_eq!(
        eval_empty_with_context_mut("a = 5.0", &mut context),
        Err(EvalexprError::expected_int(Value::from(5.0)))
    );
    // We can check which value the context stores for a like this
    assert_eq!(context.get_value("a"), Some(&Value::from(5)));
    // And use the value in another expression like this
    assert_eq!(
        eval_int_with_context_mut("a = a + 2; a", &mut context),
        Ok(7)
    );
    // It is also possible to save a bit of typing by using an operator-assignment operator
    assert_eq!(eval_int_with_context_mut("a += 2; a", &mut context), Ok(9));

    // Variables and functions
    let context = context_map! {
        "five" => 5,
        "twelve" => 12,
        "f" => Function::new(|argument| {
            if let Ok(int) = argument.as_int() {
                Ok(Value::Int(int / 2))
            } else if let Ok(float) = argument.as_float() {
                Ok(Value::Float(float / 2.0))
            } else {
                Err(EvalexprError::expected_number(argument.clone()))
            }
        }),
        "avg" => Function::new(|argument| {
            let arguments = argument.as_tuple()?;

            if let (Value::Int(a), Value::Int(b)) = (&arguments[0], &arguments[1]) {
                Ok(Value::Int((a + b) / 2))
            } else {
                Ok(Value::Float((arguments[0].as_number()? + arguments[1].as_number()?) / 2.0))
            }
        })
    }
    .unwrap(); // Do proper error handling here

    assert_eq!(
        eval_with_context("five + 8 > f(twelve)", &context),
        Ok(Value::from(true))
    );
    // `eval_with_context` returns a variant of the `Value` enum,
    // while `eval_[type]_with_context` returns the respective type directly.
    // Both can be used interchangeably.
    assert_eq!(
        eval_boolean_with_context("five + 8 > f(twelve)", &context),
        Ok(true)
    );
    assert_eq!(
        eval_with_context("avg(2, 4) == 3", &context),
        Ok(Value::from(true))
    );

    // Precompile

    let precompiled = build_operator_tree("a * b - c > 5").unwrap(); // Do proper error handling here

    let mut context = context_map! {
        "a" => 6,
        "b" => 2,
        "c" => 3
    }
    .unwrap(); // Do proper error handling here
    assert_eq!(
        precompiled.eval_with_context(&context),
        Ok(Value::from(true))
    );

    context.set_value("c".into(), 8.into()).unwrap(); // Do proper error handling here
    assert_eq!(
        precompiled.eval_with_context(&context),
        Ok(Value::from(false))
    );
    // `Node::eval_with_context` returns a variant of the `Value` enum,
    // while `Node::eval_[type]_with_context` returns the respective type directly.
    // Both can be used interchangeably.
    assert_eq!(precompiled.eval_boolean_with_context(&context), Ok(false));
}
