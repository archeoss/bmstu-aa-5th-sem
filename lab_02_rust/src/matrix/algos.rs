pub trait MatrixMultiplication {
    fn naive_mut(&self, m2: &Vec<Vec<i32>>) -> Vec<Vec<i32>>;
    fn winograd_mut(&self, m2: &Vec<Vec<i32>>) -> Vec<Vec<i32>>;
    fn winograd_mut_improved(&self, m2: &Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

trait Misc
{
    fn precompute_rows(&self) -> Vec<i32>;
    fn precompute_cols(&self) -> Vec<i32>;

    fn precompute_rows_imp(&self) -> Vec<i32>;
    fn precompute_cols_imp(&self) -> Vec<i32>;
}

impl Misc for Vec<Vec<i32>>
{
    fn precompute_rows(&self) -> Vec<i32>
    {
        let mut row_factor = vec![0; self.len()];
        for i in 0..self.len()
        {
            for j in 0..((self[0].len())/2)
            {
                row_factor[i] = row_factor[i] + self[i][j*2] * self[i][j*2 + 1];
            }
        }

        row_factor
    }

    fn precompute_cols(&self) -> Vec<i32> {
        let mut col_factor = vec![0; self[0].len()];
        for i in 0..self[0].len()
        {
            for j in 0..((self.len())/2)
            {
                col_factor[i] = col_factor[i] + self[j*2][i] * self[j*2 + 1][i];
            }
        }

        col_factor
    }

    fn precompute_rows_imp(&self) -> Vec<i32> {
        let mut row_factor = vec![0; self.len()];
        for i in 0..self.len()
        {
            for j in 0..((self[0].len())/2)
            {
                row_factor[i] += self[i][j<<1] * self[i][(j<<1) + 1];
            }
        }

        row_factor
    }

    fn precompute_cols_imp(&self) -> Vec<i32> {
        let mut col_factor = vec![0; self[0].len()];
        for i in 0..self[0].len()
        {
            for j in 0..((self.len())/2)
            {
                col_factor[i] += self[j<<1][i] * self[(j<<1) + 1][i];
            }
        }

        col_factor
    }
}

impl MatrixMultiplication for Vec<Vec<i32>> {
    fn naive_mut(&self, m2: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; m2[0].len()]; self.len()];
        for i in 0..self.len() {
            for j in 0..m2[0].len() {
                for k in 0..m2.len() {
                    result[i][j] += self[i][k] * m2[k][j];
                }
            }
        }

        result
    }

    fn winograd_mut(&self, m2: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; m2[0].len()]; self.len()];

        let row_factor = self.precompute_rows();
        let col_factor = m2.precompute_cols();

        for i in 0..result.len()
        {
            for j in 0..result[0].len()
            {
                result[i][j] = -row_factor[i] - col_factor[j];
                for k in 0..(m2.len() / 2)
                {
                    result[i][j] = result[i][j] + (self[i][k*2] + m2[k*2+1][j]) * (self[i][k*2+1] + m2[k*2][j]);
                }
            }
        }

        if m2.len()%2 == 1
        {
            for i in 0..result.len()
            {
                for j in 0..result[0].len()
                {
                    result[i][j] = result[i][j] + self[i][m2.len()-1] * m2[m2.len()-1][j]
                }
            }
        }

        result
    }

    fn winograd_mut_improved(&self, m2: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (a, b, c) = (self.len(), m2.len(), m2[0].len());
        let mut result = vec![vec![0; c]; a];

        let row_factor = self.precompute_rows_imp();
        let col_factor = m2.precompute_cols_imp();

        for i in 0..a
        {
            for j in 0..c
            {
                result[i][j] = -row_factor[i] - col_factor[j];
                for k in 0..(m2.len() / 2)
                {
                    result[i][j] += (self[i][k<<1] + m2[(k<<1)+1][j]) * (self[i][(k<<1)+1] + m2[k<<1][j]);
                }
            }
        }

        if m2.len()%2 == 1
        {
            for i in 0..a
            {
                for j in 0..c
                {
                    result[i][j] += self[i][b-1] * m2[b-1][j]
                }
            }
        }

        result
    }
}

