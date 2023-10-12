use patternfly_yew::prelude::*;
use spog_model::prelude::{PackageRelatedToProductCve, ProductCveStatus, ProductRelatedToCve};
use std::collections::BTreeMap;
use std::rc::Rc;
use yew::prelude::*;

use crate::pages::search::PaginationWrapped;

struct TableData {
    sbom_id: String,
    status: ProductCveStatus,
    packages: Vec<PackageRelatedToProductCve>,
}

trait StatusLabel {
    fn label(&self) -> &'static str;
    fn color(&self) -> Color;
}

impl StatusLabel for ProductCveStatus {
    fn label(&self) -> &'static str {
        match self {
            Self::Fixed => "Fixed",
            Self::FirstFixed => "First fixed",
            Self::FirstAffected => "First Affected",
            Self::KnownAffected => "Known affected",
            Self::LastAffected => "Last affected",
            Self::KnownNotAffected => "Knwon not affected",
            Self::Recommended => "Recommended",
            Self::UnderInvestigation => "Under investigation",
        }
    }

    fn color(&self) -> Color {
        match self {
            Self::Fixed => Color::Green,
            Self::FirstFixed => Color::Green,
            Self::FirstAffected => Color::Red,
            Self::KnownAffected => Color::Red,
            Self::LastAffected => Color::Red,
            Self::KnownNotAffected => Color::Blue,
            Self::Recommended => Color::Orange,
            Self::UnderInvestigation => Color::Grey,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Column {
    Name,
    Version,
    Status,
    Dependencies,
    Supplier,
    ReleasedOn,
}

impl TableEntryRenderer<Column> for TableData {
    fn render_cell(&self, context: CellContext<'_, Column>) -> Cell {
        match context.column {
            Column::Name => html!({
                {
                    &self.sbom_id
                }
            }),
            Column::Version => html!(<></>),
            Column::Status => html!(
                <>
                    <Label label={self.status.label().to_string()} color={self.status.color()}/>
                </>
            ),
            Column::Dependencies => html!(
                <>
                    {&self.packages.len()}
                </>
            ),
            Column::Supplier => html!(<></>),
            Column::ReleasedOn => html!(<></>),
        }
        .into()
    }
}

#[derive(PartialEq, Properties)]
pub struct RelatedProductsProperties {
    pub products: Rc<BTreeMap<ProductCveStatus, Vec<ProductRelatedToCve>>>,
}

#[function_component(RelatedProducts)]
pub fn related_products(props: &RelatedProductsProperties) -> Html {
    let table_data = use_memo(props.products.clone(), |map| {
        map.iter()
            .flat_map(|(map_key, map_value)| {
                map_value
                    .iter()
                    .map(|item| TableData {
                        status: map_key.clone(),
                        sbom_id: item.sbom_id.to_string(),
                        packages: item.packages.clone(),
                    })
                    .collect::<Vec<TableData>>()
            })
            .collect::<Vec<TableData>>()
    });

    let total = table_data.len();
    let (entries, onexpand) = use_table_data(MemoizedTableModel::new(table_data));

    let header = html_nested! {
        <TableHeader<Column>>
            <TableColumn<Column> label="Name" index={Column::Name} />
            <TableColumn<Column> label="Version" index={Column::Version} />
            <TableColumn<Column> label="Status" index={Column::Status} />
            <TableColumn<Column> label="Dependencies" index={Column::Dependencies} />
            <TableColumn<Column> label="Supplier" index={Column::Supplier} />
            <TableColumn<Column> label="Released on" index={Column::ReleasedOn} />
        </TableHeader<Column>>
    };

    let pagination = use_pagination(Some(total), || PaginationControl { page: 1, per_page: 10 });

    html!(
        <PaginationWrapped pagination={pagination} total={10}>
            <Table<Column, UseTableData<Column, MemoizedTableModel<TableData>>>
                {header}
                {entries}
                // mode={TableMode::Expandable}
                {onexpand}
            />
        </PaginationWrapped>
    )
}
