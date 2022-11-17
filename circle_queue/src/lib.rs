/// 环形队列结构
pub struct CircleQueue {
    // 队列最大长度
    max_size: usize,
    // 队列头部
    pub front: usize,
    // 队列尾部的后面一个位置，约定总是为空
    rear: usize,
    // 队列容器
    queue_vec: Vec<usize>,
}

impl CircleQueue {
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            front: 0,
            rear: 0,
            queue_vec: vec![0; max_size],
        }
    }

    /// 判断队列是否为空
    fn is_empty(&self) -> bool {
        // 当队首和队尾后一个元素相等时队列为空
        self.front == self.rear
    }

    /// 判断队列是否满
   fn is_full(&self) -> bool {
        (self.rear + 1) % self.max_size == self.front
    }

    /// 添加一个元素到队列
    pub fn add_one(&mut self, num: usize) {
        if self.is_full() {
            println!("队列已满，无法增加元素！");
        } else {
            self.queue_vec[self.rear] = num;

            // 增加元素后将 rear 后移，这里必须考虑去模
            self.rear = (self.rear + 1) % self.max_size;
        }
    }
    /// 获取队列的数据，出队列
    pub fn get_one(&mut self) -> Option<usize> {
        if self.is_empty() {
            println!("队列为空！");
            None
        } else {
            // 1.先保存原来的队首元素到临时变量
            let temp = self.queue_vec[self.front];

            // 2.再后移 front
            self.front = (self.front + 1) % self.max_size;

            // 3.最后返回出队的元素
            Some(temp)

        }
    }
    /// 显示队列的所有数据
    pub fn show_queue(&self) {
        if self.is_empty() {
            println!("队列为空！");
        } else {
            let  mut i = self.front;
            let mut j = 1;
            while i < self.front + self.actual_size() {
                println!("队列第 {n} 个是 {v}", n = j, v = self.queue_vec[i]);
                j += 1;
                i += 1;
            }
        }
    }
    /// 显示队尾元素
    pub fn get_ender(&self) -> Option<usize> {
        if self.is_empty() {
            None
        } else {
            Some(self.queue_vec[self.rear - 1])
        }
    }

    /// 显示队首元素，注意：不是取数据
    pub fn get_header(&self) -> Option<usize> {
        if self.is_empty() {
            None
        } else {
            Some(self.queue_vec[self.front])
        }
    }
    /// 求出当前队列有效数据的个数
    pub fn actual_size(&self) -> usize {
        (self.rear + self.max_size - self.front) % self.max_size
    }
}