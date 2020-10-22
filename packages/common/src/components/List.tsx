import React, { ReactElement } from "react";

function List<T>({
  data,
  pagination,
  renderItem,
  currentIndex,
  onPressPaginationIndex,
}: {
  data: T[];
  pagination?: number;
  renderItem: (item: T) => ReactElement;
  currentIndex?: number;
  onPressPaginationIndex?: (index: number) => void;
}): ReactElement {
  console.log(data.slice(0, pagination));
  //const [currentIndex, setCurrentIndex] = useState(1);

  let items;

  if (pagination !== undefined && pagination < data.length) {
    const start = (currentIndex - 1) * pagination;
    const end = currentIndex * pagination;
    items = data.slice(start, end).map((e) => renderItem(e));
  } else {
    items = data.map((e) => renderItem(e));
  }

  return (
    <div>
      <ul>{items}</ul>
      <ul>
        {pagination !== undefined &&
          pagination < data.length &&
          renderPagination(
            Math.ceil(data.length / pagination) + 1,
            //setCurrentIndex,
            onPressPaginationIndex
          )}
      </ul>
    </div>
  );
}

const renderPagination = (l, onPressPaginationIndex) => {
  const rows = [];
  for (let step = 1; step < l; step++) {
    // Runs 5 times, with values of step 0 through 4.
    console.log("Walking east one step");
    rows.push(
      <li key={step}>
        <button
          onClick={() => {
            if (onPressPaginationIndex) {
              onPressPaginationIndex(step);
            }
            //setPageIndex(step);
          }}
        >
          {step}
        </button>
      </li>
    );
  }
  return rows;
};

export default List;

// length = 5
// pagination 2

// index 0
// index*(floor(5/2)), floor(5/2)*(index+1)
// 0, 2
// 2,
