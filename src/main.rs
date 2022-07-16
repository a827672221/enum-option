fn main() {
  // 声明u32类型的整数集合
  let number_list = vec![1,2,3];
  // 调用summation求和函数
  let result = summation(&number_list);
  // 打印结果
  println!("{:?}", result);
}
// 定义求和函数
pub fn summation(list: &[u32])-> Option<u32> {
  // 声明一个0的变量来求和
  let mut sum = 0u32;
  // 声明is_overflow的布尔值来判断是否溢出
  let mut is_overflow = false;
  // for循环整数集合
  for &item in list {
    // if判断是否溢出
    if sum.checked_add(item) != None {
      // checked_add不返回None则可以相加
      sum = sum + item;
    } else {
      // checked_add返回None则溢出 is_overflow赋予true
      is_overflow = true;
      // break退出循环
      break
    }
  }
  // if判断is_overflow
  if is_overflow {
    // 为true则溢出,返回None
    None
  } else {
    // 为false则不溢出
    Some(sum)
  }
}