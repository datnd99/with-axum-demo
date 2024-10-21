# Project Structure

```
root
│
├── .vscode                    # Thư mục chứa cấu hình cho Visual Studio Code
│
├── docs                       # Thư mục chứa tài liệu dự án
│
├── logs                       # Thư mục chứa các file log (nhật ký) của ứng dụng
│
├── settings                   # Thư mục chứa các file cấu hình khác của ứng dụng
│
├── src                        # Thư mục chính chứa mã nguồn của ứng dụng
│   ├── bin                    # Thư mục chứa các module nhị phân (nếu có)
│   │
│   ├── configure              # Thư mục chứa các file cấu hình cho ứng dụng
│   │
│   ├── constant               # Thư mục chứa các hằng số sử dụng trong ứng dụng
│   │
│   ├── entities               # Thư mục chứa các thực thể (entities) của ứng dụng
│   │
│   ├── error                  # Thư mục chứa các định nghĩa lỗi và xử lý lỗi
│   │
│   ├── handler                # Thư mục chứa các handler cho các yêu cầu (request)
│   │
│   ├── migration              # Thư mục chứa các file migration cho cơ sở dữ liệu
│   │
│   ├── repositories           # Thư mục chứa các repository tương tác với cơ sở dữ liệu
│   │
│   ├── router                 # Thư mục chứa định nghĩa router cho ứng dụng
│   │
│   ├── server                 # Thư mục chứa logic khởi động server
│   │
│   ├── service                # Thư mục chứa các service xử lý logic nghiệp vụ
│   │
│   └── lib.rs                 # File chính cho module Rust (nếu sử dụng Rust)
│
├── .gitignore                 # File liệt kê các file/directory sẽ bị bỏ qua khi commit vào git
│
├── Cargo.lock                 # File khóa các phiên bản phụ thuộc (dependencies) trong Rust
│
├── Cargo.toml                 # File cấu hình dự án Rust, khai báo các phụ thuộc và metadata
│
├── LICENSE                    # File chứa thông tin giấy phép sử dụng dự án
│
└── README.md                  # Tài liệu hướng dẫn sử dụng và thông tin về dự án

```