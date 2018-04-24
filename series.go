package emseries

import (
	"errors"
	"time"
)

type Record interface {
	timestamp() time.Time
	tags() []string
}

type Series struct {
}

/*
put
search
delete
*/

func (self *Series) put(r *Record) error {
	return errors.New("[Series.put] not implemented")
}

func (self *Series) search(criteria *Criteria) ([]Record, error) {
	return []Record{}, errors.New("[Series.search] not implemented")
}

func (self *Series) remove(criteria *Criteria) ([]Record, error) {
	return []Record{}, errors.New("[Series.remove] not implemented")
}
