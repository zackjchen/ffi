
# rust create node.js package

# rust pyo3 & maturin with uv
1. 创建一个项目
```shell
uv init project_name --build-backend maturin

cd project_name
```

2. 创建运行环境,并且切换到当前环境
```sh
conda deactivate #停用conda
uv venv # 退出环境直接 deactivate

source .venv/bin/activate
```

3. 修改代码后,安装这个package
```sh
maturin develop --target aarch64-apple-darwin --uv
```

4. 运行python解释器然后测试
```sh
uv run python
```
