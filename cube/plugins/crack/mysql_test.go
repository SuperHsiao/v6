package crack

import (
	"cube/model"
	"fmt"
	"testing"
)

func TestMysqlCrack(t *testing.T) {
	task := model.CrackTask{
		Ip:   "127.0.0.1",
		Port: "3306",
		Auth: model.Auth{
			User:     "root",
			Password: "123456",
		},
		CrackPlugin: "mysql",
	}
	r := MysqlCrack(task)
	fmt.Println(r)
}

func TestRedisCrack(t *testing.T) {
	task := model.CrackTask{
		Ip:   "127.0.0.1",
		Port: "6379",
		Auth: model.Auth{
			User:     "",
			Password: "123456",
		},
		CrackPlugin: "redis",
	}
	r := RedisCrack(task)
	fmt.Println(r)
}

func TestSlice(t *testing.T) {
	var UserDict = map[string][]string{
		"ftp":        {"ftp", "admin", "www", "web", "root", "db", "wwwroot", "data"},
		"mysql":      {"root", "mysql"},
		"mssql":      {"sa", "sql"},
		"smb":        {"administrator", "admin", "guest"},
		"postgresql": {"postgres", "admin"},
		"ssh":        {"root", "admin"},
		"mongodb":    {"root", "admin"},
	}
	fmt.Println(UserDict["aa"])
}

func TestFtpCrack(t *testing.T) {
	task := model.CrackTask{
		Ip:   "127.0.0.1",
		Port: "21",
		Auth: model.Auth{
			User:     "root",
			Password: "root",
		},
		CrackPlugin: "ftp",
	}
	r := FtpCrack(task)
	fmt.Println(r)
}

func TestSmbCrack(t *testing.T) {
	task := model.CrackTask{
		Ip:   "172.20.7.20",
		Port: "445",
		Auth: model.Auth{
			User:     "Guest",
			Password: "",
		},
		CrackPlugin: "smb",
	}
	r := SmbCrack(task)
	fmt.Println(r)
}

func TestElasticCrack(t *testing.T) {
	task := model.CrackTask{
		Ip:   "172.20.42.212",
		Port: "9200",
		Auth: model.Auth{
			User:     "",
			Password: "",
		},
		CrackPlugin: "elastic",
	}
	r := ElasticCrack(task)
	fmt.Println(r)
}

func TestPhpmyadminCrack(t *testing.T) {
	task := model.CrackTask{
		Ip:   "http://127.0.0.1:8081/",
		Port: "",
		Auth: model.Auth{
			User:     "root",
			Password: "root",
		},
		CrackPlugin: "phpmyadmin",
	}
	r := PhpmyadminCrack(task)
	fmt.Println(r)
}

func TestHttpBasicCrack(t *testing.T) {
	task := model.CrackTask{
		Ip:   "http://127.0.0.1:8080",
		Port: "",
		Auth: model.Auth{
			User:     "root",
			Password: "root",
		},
		CrackPlugin: "httpbasic",
	}
	r := HttpBasicCrack(task)
	fmt.Println(r)
}

func TestMssqlCrack(t *testing.T) {
	task := model.CrackTask{
		Ip:   "172.16.157.129",
		Port: "1433",
		Auth: model.Auth{
			User:     "sa",
			Password: "123456aa",
		},
		CrackPlugin: "mssql",
	}
	r := MssqlCrack(task)
	fmt.Println(r)
}

func TestJenkinsCrack(t *testing.T) {
	task := model.CrackTask{
		Ip: "http://127.0.0.1:8081",
		//Port: "1433",
		Auth: model.Auth{
			User:     "admin",
			Password: "123456",
		},
		CrackPlugin: "jenkins",
	}
	r := JenkinsCrack(task)
	fmt.Println(r)
}

func TestZabbixCrack(t *testing.T) {
	task := model.CrackTask{
		Ip: "http://127.0.0.1:8081",
		//Port: "1433",
		Auth: model.Auth{
			User:     "Admin",
			Password: "zabbix",
		},
		CrackPlugin: "jenkins",
	}
	r := ZabbixCrack(task)
	fmt.Println(r.Result)
}
