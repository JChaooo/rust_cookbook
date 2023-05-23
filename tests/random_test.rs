#[cfg(test)]
mod tests {
    use rust_cookbook::algorithm;

    /**
     * @brief 测试 获取指定类型的随机数函数。
     * @param
     * @return
     */
    #[test]
    fn get_random_is_work() {
        let i8_random: i8 = algorithm::random::get_random_num();
        let i16_random: i16 = algorithm::random::get_random_num();
        let u32_random: u32 = algorithm::random::get_random_num();
        let f64_random: f64 = algorithm::random::get_random_num();

        println!("{}\n{}\n{}\n{}", i8_random, i16_random, u32_random, f64_random);
    }

    /**
     * @brief 测试 生成范围内随机数
     */
    #[test]
    fn get_rang_is_work() {
        algorithm::random::get_rang();
    }

    /**
     * @brief 测试 生成范围内随机数
     */
    #[test]
    fn get_sample_rang_is_work() {
        let _a = algorithm::random::get_sample_rang();
    }

    /**
     *@brief 测试 生成自定义随机值
     */
    #[test]
    fn get_diy_rang_is_work() {
        algorithm::random::get_diy_rang();
    }

    /**
     *@brief 测试 生成随机密码
     */
    #[test]
    fn get_ascll_str_is_work() {
        algorithm::random::get_ascll_str();
    }

    /**
     *@brief 测试 从一组用户自定义字符创建随机密码
     */
    #[test]
    fn get_pwd_from_div_is_work() {
        algorithm::random::get_pwd_from_div();
    }
}
