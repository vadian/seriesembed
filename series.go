package emseries

import (
	"errors"
	"os"
	"time"
)

type Record interface {
	Timestamp() time.Time
	Tags() []string
	Values() []string
}

type Series struct {
	root    string
	current *string
	fh      *os.File
}

/*
put
search
delete
*/

func NewSeries(path string) (*Series, error) {
	return nil, errors.New("[Series.NewSeries] not implemented")
}

func (self *Series) Put(r Record) error {
	return errors.New("[Series.Put] not implemented")
}

func (self *Series) Search(criteria Criteria) ([]Record, error) {
	return []Record{}, errors.New("[Series.Search] not implemented")
}

func (self *Series) Remove(criteria *Criteria) ([]Record, error) {
	return []Record{}, errors.New("[Series.Remove] not implemented")
}
