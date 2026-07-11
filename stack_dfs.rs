// source snippet: key=lib_stack_dfs  prefix=lib_stack_dfs

            let mut st = VecDeque::new();
            st.push_front(i);
            while !st.is_empty() {
                let v = st.pop_front().unwrap();
                if visited[v] == 1 {
                    continue;
                }

                visited[v] = 1;
                for j in 0..graph[v].len() {
                    let nv = graph[v][j];
                    if visited[nv.0] == 0 {
                        st.push_front(nv.0);
                    }
                }
            }
