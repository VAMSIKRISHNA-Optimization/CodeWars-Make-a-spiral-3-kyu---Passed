#[derive(PartialEq)]
enum Momentum
{
    Nope,
    Right,
    Down,
    Up,
    Left,
}


#[allow(non_snake_case, unused_mut, unused)]

fn spiralize(size: usize) -> Vec<Vec<i8>> 
{
    println!("{:?}", size);
    let mut solution =  vec![vec![0; size]; size];
    let mut Cur_Momentum =  Momentum::Right;
    
    let mut Point: (usize, usize) = (0, 0);
    solution[0][0] = 1;
    
    // Define the ends
    let mut right_end :usize = size - 1;
    let mut down_end  :usize = size - 1;
    let mut left_end  :usize = 0;
    let mut up_end    :usize = 2;
    
    let mut counter = 0;
    
    while Cur_Momentum != Momentum::Nope
    {
        counter += 1;
        
        if counter >= 10000
        {
            break;
        }
        
        match Cur_Momentum
        {
            Momentum::Right =>
            {
                // Termination Check
                if (Point.0 + 2 <= down_end) && (Point.1 + 2 <= right_end)
                {
                    if (solution[Point.0][Point.1 + 2] == 1) && (solution[Point.0 + 2][Point.1] == 1)
                    {
                        break;
                    }
                }
                
                // Right most end approached, so go down
                if Point.1 == right_end
                {
                    Cur_Momentum = Momentum::Down;
                    Point.0 = Point.0+1;
                    solution[Point.0][Point.1] = 1;
                }
                // Inner right end approached, so go down
                else if (Point.1 + 2 <= right_end) && (solution[Point.0][Point.1 + 2] == 1)
                {
                    Cur_Momentum = Momentum::Down;
                    Point.0 = Point.0+1;
                    solution[Point.0][Point.1] = 1;
                }
                // Right most end not approched, so go right
                else if Point.1 < right_end
                {
                    Cur_Momentum = Momentum::Right;
                    Point.1 = Point.1+1;
                    solution[Point.0][Point.1] = 1;
                }
            }
            
            Momentum::Left =>
            {
                // Termination Check
                if (Point.0 as i8 - 2 >= 0) && (Point.0 - 2 >= up_end) && (Point.1 as i8 - 2 >= 0) && (Point.1 - 2 >= left_end)
                {
                    if (solution[Point.0][Point.1 - 2] == 1) && (solution[Point.0 - 2][Point.1] == 1)
                    {
                        break;
                    }
                }
                
                // Left most end approached, so go up
                if Point.1 == left_end
                {
                    Cur_Momentum = Momentum::Up;
                    Point.0 = Point.0-1;
                    solution[Point.0][Point.1] = 1;
                }
                // Inner left end approached, so go up
                else if (Point.1 as i8 - 2 >= 0) && (Point.1 - 2 >= left_end) && (solution[Point.0][Point.1 - 2] == 1)
                //else if (Point.1 >= 2) && (solution[Point.0][Point.1 - 2] == 1) 
                {
                    Cur_Momentum = Momentum::Up;
                    Point.0 = Point.0-1;
                    solution[Point.0][Point.1] = 1;
                }
                // else if (Point.1 == 2) && (solution[Point.0][Point.1 - 2] == 1)
                // {
                //     Cur_Momentum = Momentum::Up;
                //     Point.0 = Point.0-1;
                //     solution[Point.0][Point.1] = 1;
                // }
                // Left most end not approched, so go left
                else if Point.1 > left_end
                {
                    Cur_Momentum = Momentum::Left;
                    Point.1 = Point.1-1;
                    solution[Point.0][Point.1] = 1; 
                }
            }
            
            Momentum::Down =>
            {
                // Termination Check
                if (Point.0 + 2 <= down_end) && (Point.0 as i8 - 2 > 0) &&(Point.1 - 2 >= left_end)
                {
                    if (solution[Point.0][Point.1 - 2] == 1) && (solution[Point.0 + 2][Point.1] == 1)
                    {
                        break;
                    }
                }
                
                // Down most end approached, so go left
                if Point.0 == down_end
                {
                    Cur_Momentum = Momentum::Left;
                    Point.1 = Point.1-1;
                    solution[Point.0][Point.1] = 1;
                }
                // Inner down end approached, so go left
                else if (Point.0 + 2 <= down_end) && (solution[Point.0 + 2][Point.1] == 1)
                {
                    Cur_Momentum = Momentum::Left;
                    Point.1 = Point.1-1;
                    solution[Point.0][Point.1] = 1;
                }
                // Down most end not approched, so go down
                else if Point.0 < down_end
                {
                    Cur_Momentum = Momentum::Down;
                    Point.0 = Point.0+1;
                    solution[Point.0][Point.1] = 1;
                }
            }
            
            Momentum::Up =>
            {
                // Termination Check
                if (Point.0 as i8 - 2 >= 0) && (Point.0 - 2 >= up_end) &&  (Point.1 + 2 <= right_end)
                {
                    if (solution[Point.0][Point.1 + 2] == 1) && (solution[Point.0 - 2][Point.1] == 1)
                    {
                        break;
                    }
                }
                
                // Up most end approached, so go right
                if Point.0 == up_end
                {
                    Cur_Momentum = Momentum::Right;
                    Point.1 = Point.1+1;
                    solution[Point.0][Point.1] = 1;
                }
                // Inner up end approached, so go right
                else if (Point.0 as i8 - 2 >= 0) && (Point.0 - 2 >= up_end) && (solution[Point.0 - 2][Point.1] == 1)
                {
                    Cur_Momentum = Momentum::Right;
                    Point.1 = Point.1+1;
                    solution[Point.0][Point.1] = 1;
                }
                // Up most end not approched, so go up
                else if Point.0 > up_end
                {
                    Cur_Momentum = Momentum::Up;
                    Point.0 = Point.0-1;
                    solution[Point.0][Point.1] = 1;
                }
            }
            
            Momentum::Nope => break,
            _ => break,
        }
        
    }
    if size % 2 == 0
    {
        solution[Point.0][Point.1] = 0;
    }
  
    
    for vec in &solution
    {
        for val in vec
        {
            print!("{:?}", val);
        }
        print!("\n");
    }

    
    
    
    solution
}
