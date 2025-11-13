package utils

import "golang.org/x/crypto/bcrypt"

func HashPassword(password string) (string, error) {
	saltRounds := 10
	hashedPassword, err := bcrypt.GenerateFromPassword([]byte(password), saltRounds)
	if err != nil {
		return "", err
	}
	return string(hashedPassword), nil
}

func ComparePassword(password, hashedPassword string) bool {
	err := bcrypt.CompareHashAndPassword([]byte(hashedPassword), []byte(password))
	return err == nil
}
