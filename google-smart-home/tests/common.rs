pub fn two_way_serde<
    T: std::fmt::Debug + PartialEq + serde::ser::Serialize + serde::de::DeserializeOwned,
>(
    json: &str,
    expected: T,
) {
    {
        let json = serde_json::to_string(&expected).unwrap();
        println!("json: {}", json);
        let parsed = serde_json::from_str::<T>(&json).unwrap();
        assert_eq!(parsed, expected);
    }
    {
        let parsed = serde_json::from_str::<T>(json).unwrap();
        assert_eq!(parsed, expected);
    }
}
